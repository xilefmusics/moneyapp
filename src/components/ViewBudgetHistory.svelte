<script>
    export let monthly;
    export let changeView;
    export let budgets;
</script>

<style>
    #main {
        flex-grow: 1;
    }
    h1 {
        padding-left: 1em;
    }
</style>

<div id='main' class="div-fill" on:click={() => changeView('dashboard')}>
    <h1>Budget History</h1>
    {#if monthly && budgets}
        <table>
            <tr>
                <th>Date</th>
                {#each budgets as budget, idx}
                    <th>{budget.name}</th>
                {/each}
            </tr>
            {#each monthly as month, idx}
                <tr>
                    <td>{`${("0" + month.date.getDate()).slice(-2)}.${("0" + (month.date.getMonth() + 1)).slice(-2)}.${month.date.getFullYear()}`}</td>
                    {#each budgets as budget, budget_idx}
                        <td>{(month.budgets[budget.name] / 100).toFixed(2)} €</td>
                    {/each}
                </tr>
            {/each}
        </table>
    {/if}
</div>