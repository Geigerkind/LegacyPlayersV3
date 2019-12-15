export interface APIToken {
    id: number;
    member_id: number;
    token: string;
    purpose: string;
    exp_date: number;
}
