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
                {#each Object.entries(year.months) as [m, month]}
                    <tr>
                        <td>{`${('0' + m).slice(-2)}.${y}`}</td>
                        <td>{(month.pod_change_sums.real / 100).toFixed(2)} €</td>
                        <td>{(month.income / 100).toFixed(2)} €</td>
                        <td>{(month.outcome / 100).toFixed(2)} €</td>
                        <td>{(month.pod_sums.real / 100).toFixed(2)} €</td>
                        <td>{(month.pod_sums.sum / 100).toFixed(2)} €</td>
                        <td>{(month.pod_sums.solvent / 100).toFixed(2)} €</td>
                        <td>{(month.pod_sums.insolvent / 100).toFixed(2)} €</td>
                        <td>{(month.pod_sums.debt / 100).toFixed(2)} €</td>
                    </tr>
                {/each}
                <tr class='upper-line under-line'>
                    <td class='bold'>{`${y}`}</td>
                        <td class='bold'>{(year.real_change / 100).toFixed(2)} €</td>
                        <td class='bold'>{(year.income / 100).toFixed(2)} €</td>
                        <td class='bold'>{(year.outcome / 100).toFixed(2)} €</td>
                        <td class='bold'></td>
                        <td class='bold'></td>
                        <td class='bold'></td>
                        <td class='bold'></td>
                        <td class='bold'></td>
                </tr>
            {/each}
        </table>
    {/if}
</div>