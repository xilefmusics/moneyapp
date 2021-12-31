const calcDiff = monthly => {
    let last_state = null;
    monthly.forEach(state => {
        state.overview.real_diff = state.overview.real - (last_state ? last_state.overview.real: 0);
        last_state = state;
    });
    return monthly;
};

export default calcDiff;