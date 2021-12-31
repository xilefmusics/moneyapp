<script>
    export let monthly;
    export let changeView;
    export let budgets;
</script>

<style>
    #main {
        background-color: purple;
        flex-grow: 1;
    }
</style>

<div id='main' class="div-fill">
    <h1 on:click={() => changeView('dashboard')}>Budget History</h1>
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
                    <td>{`${month.date.getFullYear()}-${month.date.getMonth() + 1}-${month.date.getDate()}`}</td>
                    {#each budgets as budget, budget_idx}
                        <td>{month.budgets[budget.name] / 100} €</td>
                    {/each}
                </tr>
            {/each}
        </table>
    {/if}
</div>