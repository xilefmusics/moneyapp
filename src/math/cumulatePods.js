const cumulatePods = (cumulated, pods) => {
    const acc = type => pods.filter(pod => pod.type == type).map(pod => cumulated.pod[pod.name]).reduce((a, b) => a + b);
    const solvent = acc('solvent');
    const insolvent = acc('insolvent');
    const debt = acc('debt');
    const sum = solvent + insolvent;
    const real = sum - debt;
    return {
        'solvent': solvent,
        'insolvent': insolvent,
        'debt': debt,
        'sum': sum,
        'real': real,
    };
}

export default cumulatePods;