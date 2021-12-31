const cumulateBudgets = (cumulated, budgets) => {
    return budgets.map(budget => cumulated.budget[budget.name]).reduce((a, b) => a + b);
}

export default cumulateBudgets;