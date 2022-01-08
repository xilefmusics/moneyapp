<script>
    export let monthly;
    export let changeView;
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
    {#if monthly}
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
            {#each monthly as month, idx}
                <tr>
                    <td>{`${("0" + month.date.getDate()).slice(-2)}.${("0" + (month.date.getMonth() + 1)).slice(-2)}.${month.date.getFullYear()}`}</td>
                    <td>{(month.overview.real_diff / 100).toFixed(2)} €</td>
                    <td>{(month.overview.incomming / 100).toFixed(2)} €</td>
                    <td>{(month.overview.outgoing / 100).toFixed(2)} €</td>
                    <td>{(month.overview.real / 100).toFixed(2)} €</td>
                    <td>{(month.overview.sum / 100).toFixed(2)} €</td>
                    <td>{(month.overview.solvent / 100).toFixed(2)} €</td>
                    <td>{(month.overview.insolvent / 100).toFixed(2)} €</td>
                    <td>{(month.overview.debt / 100).toFixed(2)} €</td>
                </tr>
                {#if month.lastYear}
                    <td>{month.lastYear.year}</td>
                    <td>{(month.lastYear.real_diff / 100).toFixed(2)} €</td>
                    <td>{(month.lastYear.incomming / 100).toFixed(2)} €</td>
                    <td>{(month.lastYear.outgoing / 100).toFixed(2)} €</td>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                {/if}
            {/each}
        </table>
    {/if}
</div>