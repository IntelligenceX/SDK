<?php
error_reporting(E_ALL);
ini_set('display_errors', true);

require_once realpath(dirname(__FILE__) . '/lib') . '/IntelligentSearchRequest.php';
require_once realpath(dirname(__FILE__) . '/lib') . '/IntelligentSearchResult.php';
require_once realpath(dirname(__FILE__) . '/lib') . '/searchAPI.php';

define('LOG_API', false);
define('LOG_DIR', realpath(dirname(__FILE__) . '/log'));
$term = filter_input(INPUT_GET, 'searchField');
?>
<!DOCTYPE html>
<html lang="en">

<head>
	<meta charset="UTF-8">
	<title>Intelx.io - search results</title>
</head>

<body>


<form>
	<fieldset>
		<legend>Enter your search phrase</legend>
		<input type="search" name="searchField" placeholder="Search topics or keywords" size="50" value="<?php echo htmlspecialchars($term) ?>">
		<button type="submit" id="btnSearch">Search</button>
	</fieldset>
</form>

<div id="searchResults">
<?php
if ('' != $term):
	$api = new searchAPI();
	$api->setApiKey('00000000-0000-0000-0000-000000000000'); // change the API key here
	$api->setApiUrl('https://2.intelx.io/');
	$request = new IntelligentSearchRequest($api);
	$request->search($term);
	foreach ($request->getResults($term) as $record): ?>
		<h3><?php echo $record->name ?> </h3>
		<?php echo $record->date ?><br><pre><?php echo $record->filePreview() ?></pre><br><a href="https://intelx.io/?did=<?php echo $record->systemid ?>" target="_blank">Full Data</a><hr>
	
	<?php endforeach; ?>
<?php endif; ?>
</div>

</body>

</html>