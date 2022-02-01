<script>
    export let changeView;
    export let pods;
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
    <h1>Pod History</h1>
    {#if data && pods}
        <table>
            <tr>
                <th>Date</th>
                {#each pods as pod}
                    <th>{pod.name}</th>
                {/each}
            </tr>
            {#each Object.entries(data) as [y, year]}
                {#each Object.entries(year) as [m, month]}
                    <tr>
                        <td>{`${('0' + m).slice(-2)}.${y}`}</td>
                        {#each pods as pod}
                            <td>{(month.pod[pod.name] / 100).toFixed(2)} €</td>
                        {/each}
                    </tr>
                {/each}
            {/each}
        </table>
    {/if}
</div>