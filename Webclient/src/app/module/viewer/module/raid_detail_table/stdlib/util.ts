function detail_row_post_processing(ability_details: any): void {
    // @ts-ignore
    for (const [ability, hit_types] of ability_details) {
        let total_count = 0;
        let total_amount = 0;
        for (const [hit_type, details] of hit_types) {
            total_count += details.count;
            total_amount += details.amount;
        }

        for (const [hit_type, details] of hit_types) {
            details.amount_percent = 100 * (details.amount / total_amount);
            details.count_percent = 100 * (details.count / total_count);
            details.average = details.amount / details.count;
        }
    }
}

export {detail_row_post_processing};
