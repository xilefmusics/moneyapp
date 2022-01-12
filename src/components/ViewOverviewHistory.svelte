<script>
    export let changeView;
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

<div id='main' on:click={() => changeView('dashboard')}>
    <h1>Overview History</h1>
    {#if data}
        <table>
            <tr>
                <th>Date</th>
                <th>Change</th>
                <th>In</th>
                <th>Out</th>
                <th>Real</th>
                <th>Sum</th>
                <th>Solvent</th>
                <th>Insolvent</th>
                <th>Debt</th>
            </tr>
            {#each Object.entries(data) as [y, year]}
                {#each Object.entries(year) as [m, month]}
                    <tr>
                        <td>{`${('0' + m).slice(-2)}.${y}`}</td>
                        <td>{(month[0].pod_change_sums.real / 100).toFixed(2)} €</td>
                        <td>{(month[0].income / 100).toFixed(2)} €</td>
                        <td>{(month[0].outcome / 100).toFixed(2)} €</td>
                        <td>{(month[0].pod_sums.real / 100).toFixed(2)} €</td>
                        <td>{(month[0].pod_sums.sum / 100).toFixed(2)} €</td>
                        <td>{(month[0].pod_sums.solvent / 100).toFixed(2)} €</td>
                        <td>{(month[0].pod_sums.insolvent / 100).toFixed(2)} €</td>
                        <td>{(month[0].pod_sums.debt / 100).toFixed(2)} €</td>
                    </tr>
                {/each}
            {/each}
        </table>
    {/if}
</div>