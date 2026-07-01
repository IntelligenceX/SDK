use intelx::IdentityClient;

use crate::cli::{IdentityArgs, IdentityKind};
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

fn write_tsv(filename: &str, headers: &[&str], rows: &[Vec<String>]) -> std::io::Result<()> {
    use std::io::Write;
    let mut file = std::fs::File::create(filename)?;
    writeln!(file, "{}", headers.join("\t"))?;
    for row in rows {
        writeln!(file, "{}", row.join("\t"))?;
    }
    Ok(())
}

pub async fn run(client: &IdentityClient, args: IdentityArgs, raw: bool) -> intelx::Result<()> {
    let buckets = parse_buckets(&args.buckets);

    match args.kind {
        IdentityKind::DataLeaks => {
            if !raw {
                output::info(&format!("Starting data leaks search of \"{}\".", args.term));
            }
            let mut params = intelx::IdSearchParams::new(&args.term).maxresults(args.limit);
            if let Some(bucket) = buckets.first() {
                params = params.bucket(bucket.clone());
            }
            if let Some(datefrom) = &args.datefrom {
                params = params.datefrom(datefrom.clone());
            }
            if let Some(dateto) = &args.dateto {
                params = params.dateto(dateto.clone());
            }
            let records = client.idsearch(params).await?;

            if raw {
                output::print_json(&records);
                return Ok(());
            }

            let rows: Vec<Vec<String>> = records
                .iter()
                .filter_map(|r| r.item.as_ref())
                .map(|item| vec![item.name.clone(), item.date.clone(), item.bucket.clone()])
                .collect();
            output::table(vec!["Name", "Date", "Bucket"], rows.clone());

            let filename = format!("intelx-output-{}-data_leaks.tsv", args.term);
            write_tsv(&filename, &["Name", "Date", "Bucket"], &rows)?;
            output::info(&format!("Exported output to \"{filename}\"."));
        }
        IdentityKind::ExportAccounts => {
            if !raw {
                output::info(&format!("Starting account export of \"{}\".", args.term));
            }
            let mut params = intelx::ExportAccountsParams::new(&args.term).maxresults(args.limit);
            if let Some(datefrom) = &args.datefrom {
                params = params.datefrom(datefrom.clone());
            }
            if let Some(dateto) = &args.dateto {
                params = params.dateto(dateto.clone());
            }
            let records = client.export_accounts(params).await?;

            if raw {
                output::print_json(&records);
                return Ok(());
            }

            let rows: Vec<Vec<String>> = records
                .iter()
                .map(|r| {
                    vec![
                        r.user.clone(),
                        r.password.clone(),
                        r.passwordtype.to_string(),
                        r.sourceshort.clone(),
                    ]
                })
                .collect();
            output::table(
                vec!["User", "Password", "Password Type", "Source Short"],
                rows.clone(),
            );

            let filename = format!("intelx-output-{}-export_accounts.tsv", args.term);
            write_tsv(
                &filename,
                &["User", "Password", "Password Type", "Source Short"],
                &rows,
            )?;
            output::info(&format!("Exported output to \"{filename}\"."));
        }
        IdentityKind::ReverseDomain => {
            if !raw {
                output::info(&format!(
                    "Starting reverse domain export of \"{}\".",
                    args.term
                ));
            }
            let mut params = intelx::ReverseDomainParams::new(&args.term).maxresults(args.limit);
            if let Some(datefrom) = &args.datefrom {
                params = params.datefrom(datefrom.clone());
            }
            if let Some(dateto) = &args.dateto {
                params = params.dateto(dateto.clone());
            }
            let records = client.reverse_domain(params).await?;

            if raw {
                output::print_json(&records);
                return Ok(());
            }

            let rows: Vec<Vec<String>> = records
                .iter()
                .map(|r| {
                    vec![
                        r.user.clone(),
                        r.password.clone(),
                        r.url.clone(),
                        r.sourceshort.clone(),
                    ]
                })
                .collect();
            output::table(
                vec!["User", "Password", "URL", "Source Short"],
                rows.clone(),
            );

            let filename = format!("intelx-output-{}-export_accounts.tsv", args.term);
            write_tsv(
                &filename,
                &["User", "Password", "URL", "Source Short"],
                &rows,
            )?;
            output::info(&format!("Exported output to \"{filename}\"."));
        }
    }

    Ok(())
}
