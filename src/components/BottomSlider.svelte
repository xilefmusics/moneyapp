<script>
    export let changeView;
    export let view;
</script>
  
<style>
    #main {
        display: flex;
        background-color: #303030;
    }
    .item {
        flex: 1;
        text-align: center;
        font-weight: bold;
        font-size: 3em;
    }
    .selected {
        color: #DDDDDD;
    }
    .not-selected {
        color: #AAAAAA;
    }
</style>

<div id='main'>
    <div class={`item ${view === 'dashboard' ? 'selected' : 'not-selected'}`}
        on:click={() => changeView('dashboard')}
    >D</div>
    <div class={`item ${view === 'pod' ? 'selected' : 'not-selected'}`}
        on:click={() => changeView('pod')}
    >P</div>
    <div class={`item ${view === 'addBooking' ? 'selected' : 'not-selected'}`}
        on:click={() => changeView('addBooking')}
    >+</div>
    <div class={`item ${view === 'budget' ? 'selected' : 'not-selected'}`}
        on:click={() => changeView('budget')}
    >B</div>
    <div class={`item ${view === 'config' ? 'selected' : 'not-selected'}`}
        on:click={() => changeView('config')}
    >C</div>
</div>