<script>
    export let changeView;
</script>
  
<style>
    #main {
        background-color: blue;
        display: flex;
    }
    .item {
        border-left: solid;
        flex: 1;
        text-align: center;
    }
</style>

<div id='main'>
    <div class='item'
        on:click={() => changeView('dashboard')}
    >Dashboard</div>
    <div class='item'
        on:click={() => changeView('pod')}
    >Pod</div>
    <div class='item'
        on:click={() => changeView('budget')}
    >Budget</div>
    <div class='item'
        on:click={() => changeView('booking')}
    >Booking</div>
</div>