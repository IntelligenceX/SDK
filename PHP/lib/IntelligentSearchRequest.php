<?php

class IntelligentSearchRequest
{
	//Status
	const STATUS_SUCCESS = 0; // Success with results,
	const STATUS_NO_MORE_RESULTS = 1; // No more results available,
	const STATUS_ID_NOT_FOUND = 2; // Search ID not found,
	const STATUS_NO_RESULTS_YET = 3; // No results yet available keep trying

	//Sort
	const SORT_NO_SORTING = 0; // no sorting
	const SORT_XSCORE_ASC = 1; // X-Score ASC
	const SORT_XSCORE_DESC = 2; // X-Score DESC,
	const SORT_DATE_ASC = 3; // Date ASC,
	const SORT_DATE_DESC = 4; // Date DESC
	
	//Media
	const MEDIA_NOT_SET = 0; // Not set
	const MEDIA_PASTE = 1; // Paste
	const MEDIA_PASTE_USER = 2; // Paste User
	const MEDIA_FORUM = 3; // Forum
	const MEDIA_FORUM_BOARD = 4; // Forum Board
	const MEDIA_FORUM_THREAD = 5; // Forum Thread
	const MEDIA_FORUM_POST = 6; // Forum Post
	const MEDIA_FORUM_USER = 7; // Forum User
	const MEDIA_SCREENSHOT = 8; // Screenshot of a Website
	const MEDIA_HTML_COPY = 9; // HTML copy of a Website
	const MEDIA_TEXT_COPY = 10; // Text copy of a Website
	
	private $api = null;
	protected $term;
	protected $buckets = [];
	protected $maxresults = 1000;
	protected $timeout = 0;
	protected $datefrom = "";
	protected $dateto = "";
	protected $sort = self::SORT_XSCORE_DESC;
	protected $media = self::MEDIA_NOT_SET;
	protected $terminate = [];
	protected $lastResult = null;
	
	public function __construct(searchAPI $api, $term = null)
	{
		$this->api = $api;
		
		if (null !== $term) {
			$this->setTerm($term);
		}
	}
	
	public function getSearchData()
	{
		return [
			"term" => $this->term,
			"buckets" => $this->buckets,
			"maxresults" => $this->maxresults,
			"timeout" => $this->timeout,
			"datefrom" => $this->datefrom,
			"dateto" => $this->dateto,
			"sort" => $this->sort,
			"media" => $this->media,
			"terminate" => $this->terminate,
		];
	}
	
	public function search($term = null)
	{
		if (null !== $term) {
			$this->setTerm($term);
		}
		
		$this->lastResult = $this->api->search($this->getSearchData());
	}
	
	public function hasResult()
	{
		return is_array($this->lastResult) && isset($this->lastResult['id']) && $this->lastResult['status'] == self::STATUS_SUCCESS;
	}
	
	public function getResults($limit = 100, $offset = 0, $previewlines = 8)
	{
		if ($this->hasResult()) {
			$query = [
				"id" => $this->lastResult['id'],
				"limit" => $limit,
				"offset" => $offset,
				"previewlines" => $previewlines,
			];
			
			$searchResult = $this->api->searchResult($query);
			$result = [];
			foreach ($searchResult["records"] as $line) {
				$result[] = new IntelligentSearchResult($this->api, $line);
			}
			
			return $result;
		}
	}
	
	public function terminate()
	{
		if ($this->hasResult()) {
			$this->api->searchTerminate($this->lastResult['id']);
		}
	}
	
		/**
	 * @param mixed $term
	 */
	public function setTerm($term)
	{
		$this->term = $term;
	}
	
	/**
	 * @param array $buckets
	 */
	public function setBuckets($buckets)
	{
		$this->buckets = $buckets;
	}
	
	/**
	 * @param int $maxresults
	 */
	public function setMaxresults($maxresults)
	{
		$this->maxresults = $maxresults;
	}
	
	/**
	 * @param int $timeout
	 */
	public function setTimeout($timeout)
	{
		$this->timeout = $timeout;
	}
	
	/**
	 * @param string $datefrom
	 */
	public function setDatefrom($datefrom)
	{
		$this->datefrom = $datefrom;
	}
	
	/**
	 * @param string $dateto
	 */
	public function setDateto($dateto)
	{
		$this->dateto = $dateto;
	}
	
	/**
	 * @param int $sort
	 */
	public function setSort($sort)
	{
		$this->sort = $sort;
	}
	
	/**
	 * @param int $media
	 */
	public function setMedia($media)
	{
		$this->media = $media;
	}
	
	/**
	 * @param array $terminate
	 */
	public function setTerminate($terminate)
	{
		$this->terminate = $terminate;
	}
}