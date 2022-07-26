fn use_move(move, user, targets) {
    let results = [];
    for target in targets {
        switch user.throw_move(random, move) {
            false => results.push(Miss(user)),
            true => {
                let result = user.damage(random, target, 40, move.category, move.type, move.crit_rate);
                if result.damage > target.hp {
                    result.damage = target.hp - 1;
                }
                results.push(Damage(target, result));
            }
        }
    }
    results
}