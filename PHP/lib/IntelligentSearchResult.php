<?php

class IntelligentSearchResult
{
	private $api = null;
	protected $data;
	
	public function __construct(searchAPI $api, $data)
	{
		$this->api = $api;
		$this->data = $data;
	}
	
	public function __get($name)
	{
		return isset($this->data[$name]) ? $this->data[$name] : null;
	}
	
	public function fileRead()
	{
		return $this->api->fileRead($this->storageid, $this->systemid);
	}
	
	public function fileView()
	{
		return $this->api->fileView($this->storageid);
	}

	public function filePreview()
	{
		return $this->api->filePreview($this->storageid);
	}
}