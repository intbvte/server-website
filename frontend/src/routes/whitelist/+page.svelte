<script lang="ts">
	const backendUrl = 'http://localhost:8000/backend/';
</script>

<form action={backendUrl + 'minecraft/username/change'} method="post">
	<input type="text" name="username" />
	<button type="submit">add to whitelist</button>
</form>
