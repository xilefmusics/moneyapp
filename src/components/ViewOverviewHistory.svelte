<script>
    export let monthly;
    export let changeView;
</script>

<style>
    #main {
        background-color: purple;
        flex-grow: 1;
    }
</style>

<div id='main' class="div-fill">
    <h1 on:click={() => changeView('dashboard')}>Overview History</h1>
    {#if monthly}
        <table>
            <tr>
                <th>Date</th>
                <th>Real</th>
                <th>Sum</th>
                <th>Solvent</th>
                <th>Insolvent</th>
                <th>Debt</th>
                <th>Change</th>
            </tr>
            {#each monthly as month, idx}
                <tr>
                    <td>{`${month.date.getFullYear()}-${month.date.getMonth() + 1}-${month.date.getDate()}`}</td>
                    <td>{month.overview.real / 100} €</td>
                    <td>{month.overview.sum / 100} €</td>
                    <td>{month.overview.solvent / 100} €</td>
                    <td>{month.overview.insolvent / 100} €</td>
                    <td>{month.overview.debt / 100} €</td>
                    <td>{month.overview.real_diff / 100} €</td>
                </tr>
            {/each}
        </table>
    {/if}
</div>