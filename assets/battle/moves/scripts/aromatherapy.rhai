fn use_move(move, user, targets) {
    let results = [];

    switch user.throw_move(random, move) {
        false => {
            results.push(Miss(user));
        },
        true => {
            for target in targets {
                results.push(Ailment(target, CLEAR));
            }
        }
    }

    results
}