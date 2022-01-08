<script>
    export let state;
    export let changeView;
</script>

<style>
    #main {
        flex-grow: 1;
    }
</style>

<div id='main' class="div-fill div-scroll">
    {#if state}
      <div class='box' on:click={() => changeView('overviewHistory')}>
        <h2>Overview</h2>
        <table class='div-fill'>
            <tr><td class='left no-pad'>Solvent</td><td class='right no-pad'>{(state.overview.solvent / 100).toFixed(2)} €</td></tr>
            <tr><td class='left no-pad'>Insolvent</td><td class='right no-pad'>+ {(state.overview.insolvent / 100).toFixed(2)} €</td></tr>
            <tr class='upper-line'><td class='left no-pad'>Sum</td><td class='right no-pad'>{(state.overview.sum / 100).toFixed(2)} €</td></tr>
            <tr><td class='left no-pad'>Debt</td><td class='right no-pad'>- {(state.overview.debt / 100).toFixed(2)} €</td></tr>
            <tr class='upper-line'><td class='left no-pad'>Real</td><td class='right bold no-pad'>{(state.overview.real / 100).toFixed(2)} €</td></tr>
          </table>
          <h3>In/Out</h3>
          <table class='div-fill'>
            <tr><td class='left no-pad'>Incomming</td><td class='right no-pad'>{(state.overview.incomming / 100).toFixed(2)} €</td></tr>
            <tr><td class='left no-pad'>Outgoing</td><td class='right no-pad'>{(state.overview.outgoing / 100).toFixed(2)} €</td></tr>
            <tr><td class='left no-pad'>Change</td><td class='right no-pad'>{(state.overview.real_diff / 100).toFixed(2)} €</td></tr>
          </table>
      </div>

      <div class='box' on:click={() => changeView('budgetHistory')}>
        <h2>Budgets</h2>
        <table class='div-fill'>
            {#each Object.entries(state.budgets) as budget, idx}
                <tr>
                    <td class='left no-pad'>{budget[0]}</td>
                    <td class='right no-pad'>{(budget[1] / 100).toFixed(2)} €</td>
                </tr>
          {/each}
          <tr class='upper-line'>
            <td class='left no-pad'>Sum</td>
            <td class='right bold no-pad'>{(state.budget_sum / 100).toFixed(2)} €</td>
          </tr>
        </table>
      </div>

      <div class='box' on:click={() => changeView('podHistory')}>
        <h2>Pods</h2>
        <table class='div-fill'>
            {#each Object.entries(state.pods) as pod, idx}
                <tr>
                    <td class='left no-pad'>{pod[0]}</td>
                    <td class='right no-pad'>{(pod[1] / 100).toFixed(2)} €</td>
                </tr>
          {/each}
        </table>
      </div>

    {/if}
</div>
