<script>
    export let changeView;
    export let budgets;
    export let data;
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
    {#if data && budgets}
        <table>
            <tr>
                <th>Date</th>
                {#each budgets as budget}
                    <th>{budget.name}</th>
                {/each}
            </tr>
            {#each Object.entries(data) as [y, year]}
                {#each Object.entries(year.months) as [m, month]}
                    <tr>
                        <td>{`${('0' + m).slice(-2)}.${y}`}</td>
                        {#each budgets as budget}
                            <td>{(month.budget[budget.name] / 100).toFixed(2)} €</td>
                        {/each}
                    </tr>
                {/each}
            {/each}
        </table>
    {/if}
</div>