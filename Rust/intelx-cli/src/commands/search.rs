use intelx::IntelXClient;

use crate::cli::{ExportFormatArg, PhonebookKind, SearchArgs};
use crate::output;

fn parse_buckets(buckets: &Option<String>) -> Vec<String> {
    buckets
        .as_deref()
        .map(|b| {
            b.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

pub async fn run(client: &IntelXClient, args: SearchArgs, raw: bool) -> intelx::Result<()> {
    if args.limit.is_none() && !args.stats && args.phonebook.is_none() && !raw {
        output::warn("Limit argument not supplied, setting default to 10 results.");
    }
    let limit = args.limit.unwrap_or(10);
    let buckets = parse_buckets(&args.buckets);

    if let Some(phonebook) = args.phonebook {
        return run_phonebook(client, &args, phonebook, buckets, raw).await;
    }

    if !raw {
        output::info(&format!("Starting search of \"{}\".", args.term));
    }

    let mut params = intelx::SearchParams::new(&args.term)
        .maxresults(if args.stats { 1000 } else { limit.max(100) })
        .buckets(buckets)
        .timeout(args.timeout)
        .media(args.media);
    if let Some(datefrom) = &args.datefrom {
        params = params.datefrom(datefrom.clone());
    }
    if let Some(dateto) = &args.dateto {
        params = params
            .dateto(dateto.clone())
            .sort(intelx::SortOrder::XScoreDesc);
    }

    if args.export {
        let format = match args.export_format {
            ExportFormatArg::Csv => intelx::ExportFormat::Csv,
            ExportFormatArg::Zip => intelx::ExportFormat::Zip,
        };
        let path = client
            .export_from_search(params, format, &args.out_dir)
            .await?;
        output::info(&format!(
            "Exported search results to \"{}\".",
            path.display()
        ));
        return Ok(());
    }

    let results = client.search(params).await?;

    if raw {
        output::print_json(&results);
        return Ok(());
    }

    if args.stats {
        let stats = intelx::stats(&results);
        output::print_json(&stats);
        return Ok(());
    }

    for result in results.iter().take(limit as usize) {
        let name = if result.name.is_empty() {
            "Untitled Document"
        } else {
            &result.name
        };
        println!(
            "________________________________________________________________________________"
        );
        println!("> Name: {name}");
        println!("> Date: {}", result.date);
        println!("> Size: {} bytes", result.size);
        println!("> Media: {}", result.mediah);
        println!("> Bucket: {}", result.bucketh);
        println!("> ID: {}", result.systemid);

        if args.view {
            let text = client
                .file_view(
                    result.item_type,
                    result.media,
                    &result.storageid,
                    &result.bucket,
                    0,
                )
                .await?;
            if !text.is_empty() {
                println!("\n{text}");
            }
        } else if !args.nopreview {
            let preview_params = intelx::FilePreviewParams::new(
                result.item_type,
                result.media,
                0,
                result.storageid.clone(),
            )
            .bucket(result.bucket.clone());
            let text = client.file_preview(preview_params).await?;
            if !text.is_empty() {
                println!("\n{text}");
            }
        }
        println!(
            "________________________________________________________________________________"
        );
    }

    Ok(())
}

async fn run_phonebook(
    client: &IntelXClient,
    args: &SearchArgs,
    kind: PhonebookKind,
    buckets: Vec<String>,
    raw: bool,
) -> intelx::Result<()> {
    if !raw {
        output::info(&format!("Starting phonebook search of \"{}\".", args.term));
    }

    let target = match kind {
        PhonebookKind::All => intelx::PhonebookTarget::All,
        PhonebookKind::Domains => intelx::PhonebookTarget::Domains,
        PhonebookKind::Emails => intelx::PhonebookTarget::EmailAddresses,
        PhonebookKind::Urls => intelx::PhonebookTarget::Urls,
    };

    let mut params = intelx::PhonebookSearchParams::new(&args.term)
        .maxresults(args.limit.unwrap_or(1000))
        .buckets(buckets)
        .target(target);
    if let Some(datefrom) = &args.datefrom {
        params.datefrom = datefrom.clone();
    }
    if let Some(dateto) = &args.dateto {
        params.dateto = dateto.clone();
    }

    let pages = client.phonebook_search_all(params).await?;

    if raw {
        output::print_json(&pages);
        return Ok(());
    }

    let selectors = intelx::flatten_selectors(&pages);
    if args.emails {
        for selector in &selectors {
            if selector.selectortype == 1 {
                println!("{}", selector.selectorvalue);
            }
        }
        return Ok(());
    }

    let rows = selectors
        .iter()
        .map(|s| vec![s.selectortypeh.clone(), s.selectorvalue.clone()])
        .collect();
    output::table(vec!["Type", "Value"], rows);

    Ok(())
}
