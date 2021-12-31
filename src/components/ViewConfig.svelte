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
    <input type="text" id='config-url'>
    <input type="text" id='config-user'>
    <input type="text" id='config-password'>
    <button on:click={doConfig}>config</button>
</div>
