<script>
    export let data;
    export let changeView;
</script>

<style>
    #main {
        flex-grow: 1;
    }
</style>

<div id='main' class="div-fill div-scroll">
    {#if data}
      <div class='box' on:click={() => changeView('overviewHistory')}>
        <h2>Overview</h2>
        <table class='div-fill'>
            <tr><td class='left no-pad'>Solvent</td><td class='right no-pad'>{(data.pod_sums.solvent / 100).toFixed(2)} €</td></tr>
            <tr><td class='left no-pad'>Insolvent</td><td class='right no-pad'>+ {(data.pod_sums.insolvent / 100).toFixed(2)} €</td></tr>
            <tr class='upper-line'><td class='left no-pad'>Sum</td><td class='right no-pad'>{(data.pod_sums.sum / 100).toFixed(2)} €</td></tr>
            <tr><td class='left no-pad'>Debt</td><td class='right no-pad'>- {(data.pod_sums.debt / 100).toFixed(2)} €</td></tr>
            <tr class='upper-line'><td class='left no-pad'>Real</td><td class='right bold no-pad'>{(data.pod_sums.real / 100).toFixed(2)} €</td></tr>
          </table>
          <h3>In/Out</h3>
          <table class='div-fill'>
            <tr><td class='left no-pad'>In</td><td class='right no-pad'>{(data.income / 100).toFixed(2)} €</td></tr>
            <tr><td class='left no-pad'>Out</td><td class='right no-pad'>{(data.outcome / 100).toFixed(2)} €</td></tr>
            <tr><td class='left no-pad'>Change</td><td class='right no-pad'>{(data.pod_change_sums.real / 100).toFixed(2)} €</td></tr>
          </table>
      </div>

      <div class='box' on:click={() => changeView('budgetHistory')}>
        <h2>Budgets</h2>
        <table class='div-fill'>
            {#each Object.entries(data.budget) as budget, idx}
                <tr>
                    <td class='left no-pad'>{budget[0]}</td>
                    <td class='right no-pad'>{(budget[1] / 100).toFixed(2)} €</td>
                </tr>
          {/each}
          <tr class='upper-line'>
            <td class='left no-pad'>Sum</td>
            <td class='right bold no-pad'>{( data.budget_sum / 100).toFixed(2)} €</td>
          </tr>
        </table>
      </div>

      <div class='box' on:click={() => changeView('podHistory')}>
        <h2>Pods</h2>
        <table class='div-fill'>
            {#each Object.entries(data.pod) as pod, idx}
                <tr>
                    <td class='left no-pad'>{pod[0]}</td>
                    <td class='right no-pad'>{(pod[1] / 100).toFixed(2)} €</td>
                </tr>
          {/each}
        </table>
      </div>

    {/if}
</div>
