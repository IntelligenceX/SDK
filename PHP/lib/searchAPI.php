<?php

class searchAPI
{
	const DOWNLOAD_TYPE_BINARY = 0; // Raw binary
	const DOWNLOAD_TYPE_BINARY_DISPOSITION = 1; // Raw binary with content disposition and optional file &name=[name] as parameter; Filename: "[System ID].bin" or "[Storage ID].bin" unless specified in the name parameter

	//Format:
	const FORMAT_TEXT = 0; // Text view, any non-printable characters shall be removed, UTF-8 encoding.
	const FORMAT_HEX = 1; // Hex view of data.
	const FORMAT_AUTODETECT = 2; // Auto-detect hex view or text view.
	const FORMAT_PICTURE = 3; // Picture view.
	const FORMAT_NOT_SUPPORTED = 4; // Not supported.
	const FORMAT_HTML = 5; // HTML inline view. Content will be sanitized and modified!
	const FORMAT_PDF_TEXT = 6; // Text view of PDF. Content will be automatically converted.
	const FORMAT_PDF_HTML = 7; // Text view of HTML.
	const FORMAT_WORD_TEXT = 8; // Text view of Word files (DOC/DOCX/RTF).
	
	protected $API_KEY;
	protected $API_URL;
	
	public function __construct()
	{
	}
	
	/**
	 * @param string $API_KEY
	 */
	public function setApiKey($API_KEY)
	{
		$this->API_KEY = $API_KEY;
	}
	
	/**
	 * @param string $API_URL
	 */
	public function setApiUrl($API_URL)
	{
		$this->API_URL = $API_URL;
	}
	
	/**
	 * Submits an intelligent search request
	 * /intelligent/search
	 */
	public function search($query)
	{
		return $this->call('POST', 'intelligent/search', [], $query);
	}
	
	/**
	 * Returns selected results
	 * /intelligent/search/result
	 */
	public function searchResult($query)
	{
		return $this->call('GET', 'intelligent/search/result', $query);
	}
	
	/**
	 * Terminates a search
	 * /intelligent/search/terminate
	 */
	public function searchTerminate($uuid)
	{
		return $this->call('GET', 'intelligent/search/terminate', ['id' => $uuid]);
	}
	
	/**
	 * Submits a phone book alike search
	 * /phonebook/search
	 */
	public function phonebookSearch($term)
	{
		$post = [
			"term" => $term,
			"buckets" => [],
			"maxresults" => 1000,
			"timeout" => 0,
			"datefrom" => "",
			"dateto" => "",
			"sort" => 2,
			"media" => 0,
			"terminate" => [],
		];
		
		return $this->call('POST', 'intelligent/search', [], $post);
	}

	/**
	 * Returns results
	 * /phonebook/search/result
	 */
 	public function phonebookSearchResult($query)
	{
		return $this->call('GET', 'phonebook/search/result', $query);
	}
	
	/**
	 * Reads an items data for download
	 * /file/read
	 */
	public function fileRead($storageid, $systemid, $bucket = '', $download_type = 0)
	{
		return $this->call('GET', 'file/read', [
			'type' => $download_type, 
			'storageid' => $storageid, 
			'systemid' => $systemid, 
			'bucket' => $bucket,
		]);
	}
	
	/**
	 * Reads an items data for detailed inline view
	 * /file/view
	 */
	public function fileView($storageid, $bucket = '', $format = 0)
	{
		return $this->call('GET', 'file/view', [
			'f' => $format,
			'storageid' => $storageid,
			'bucket' => $bucket,
		]);
	}
	
	/**
	 * Reads an items data for preview
	 * /file/preview
	 */
	public function filePreview($storageid, $contentType = 1, $mediaType = 1, $targetFormat = 0, $bucket = '', $e = 0)
	{
		return $this->call('GET', 'file/preview', [
			'sid' => $storageid,
			'f' => $targetFormat,
			'c' => $contentType,
			'm' => $mediaType,
			'b' => $bucket,
			'k' => $this->API_KEY,
		]);
	}
	
	
	protected function call($type, $link, $query = [], $post = null)
	{
		$url = $this->API_URL . $link;
		
		$url .= '?' . http_build_query($query);
		
		$ch = curl_init($url);
		curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
		curl_setopt($ch, CURLINFO_HEADER_OUT, true);
		$headers = ["x-key: " . $this->API_KEY];
		if ($type == 'GET') {
		} elseif ($type == 'POST') {
			curl_setopt($ch, CURLOPT_POST, 1);
			curl_setopt($ch, CURLOPT_POSTFIELDS, json_encode($post));
			$headers[] = "Content-type: application/json";
		}
		curl_setopt($ch, CURLOPT_HTTPHEADER, $headers);
		
		$server_output = curl_exec($ch);
		$status = curl_getinfo($ch);
		curl_close($ch);
		
		self::log($url, $post, $server_output, $status);
		
		switch($status["http_code"]) {
			case 200:
				$data = json_decode($server_output, true);
				return json_last_error() == JSON_ERROR_NONE ? $data : $server_output;
				break;
			case 400:
				// invalid request
				break;
			case 404:
				// unknown service
				break;
			case 500:
				// service error
				break;
			case 0:
				// host not found
				break;
		}
		
		return false;
	}
	
	protected static function log($url, $post, $server_output, $status)
	{
		if (LOG_API) {
			$text = $url . "\n";
			if (is_array($post) && count($post)) {
				$text .= "  POST: " . json_encode($post);
			}
			$text .= "\n  RESULT: " . $server_output;
			$text .= "\n  INFO: " . json_encode($status);
			
			$log_file = LOG_DIR . '/search_api_' . date('Y-m-d') . '.log';
			file_put_contents($log_file, date('Y-m-d H:i:s ') . $text . "\n\n", FILE_APPEND);
		}
	}
	
}