const initStructure = (pods, budgets) => ({
    pod: Object.fromEntries(pods.map(p => [p['name'], 0])),
    budget: Object.fromEntries(budgets.map(b => [b['name'], 0])),
});

const cumulate = (pods, budgets, bookings, from, until, init) => {
    init === undefined && (init = initStructure(pods, budgets)); 
    let state = {...init};
    state.incomming = 0;
    state.outgoing = 0;   
    bookings
        .filter(booking => (booking.date.getTime() < until.getTime() && booking.date.getTime() >= from.getTime()))
        .forEach(booking => {
            if (booking.from_pod !== 'world') { state.pod[booking.from_pod] -= booking.amount; }
            if (booking.to_pod !== 'world') { state.pod[booking.to_pod] += booking.amount; };
            if (booking.from_budget !== 'world') { state.budget[booking.from_budget] -= booking.amount; };
            if (booking.to_budget !== 'world') { state.budget[booking.to_budget] += booking.amount; };
            if (booking.from_pod === 'world' 
                && booking.from_budget === 'world'
                && booking.to_pod !== 'world'
                && booking.to_budget !== 'world') {state.incomming += booking.amount;};
            if (booking.from_pod !== 'world' 
                && booking.from_budget !== 'world'
                && booking.to_pod === 'world'
                && booking.to_budget === 'world') {state.outgoing += booking.amount;};
    });
    return state;
};

export default cumulate;