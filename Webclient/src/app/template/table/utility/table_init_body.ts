import {HeaderColumn} from "../module/table_header/domain_value/header_column";
import {BodyColumn} from "../module/table_body/domain_value/body_column";

export function init_body_columns_from_result(result: any, header: HeaderColumn[]): BodyColumn[][] {
    return result.map(row => {
        const result = [];
        header.forEach(entry => result.push({
            type: entry.type,
            content: entry.type === 3 ?
                        row[entry.filter_name].label_key.toString() :
                        (entry.type === 2 ?
                            row[entry.filter_name] * 1000 :
                            row[entry.filter_name]).toString()
        }));
        return result;
    });
}
