<script>
    const doConfig = () => {
        localStorage.setItem('url', document.getElementById('config-url').value);
        localStorage.setItem('user', document.getElementById('config-user').value);
        localStorage.setItem('password', document.getElementById('config-password').value);
    };
</script>

<style>
</style>

<div id='main' class="div-fill div-scroll">
    <h1>Config</h1>
    <h2>URL</h2>
    <input type="text" id='config-url'>
    <h2>User</h2>
    <input type="text" id='config-user'>
    <h2>Password</h2>
    <input type="text" id='config-password'>
    <button on:click={doConfig}>config</button>
</div>
