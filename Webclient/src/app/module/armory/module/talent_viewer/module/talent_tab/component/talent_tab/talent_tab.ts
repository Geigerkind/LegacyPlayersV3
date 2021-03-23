import {Component, EventEmitter, Input, OnChanges, OnInit, Output} from "@angular/core";
import {Talent} from "../../value_object/talent";

@Component({
    selector: "TalentTab",
    templateUrl: "./talent_tab.html",
    styleUrls: ["./talent_tab.scss"]
})
export class TalentTabComponent implements OnInit, OnChanges {

    @Input() tab_label: string;
    @Input() i_talent_tree: Array<Array<Talent>>;
    @Input() expansion_id: number;
    @Output() points_changed: EventEmitter<number> = new EventEmitter();

    points_spend: number = 0;
    talent_tree: Array<Array<Talent>> = [];

    constructor() {
    }

    ngOnInit(): void {
        this.talent_tree = this.i_talent_tree;
    }

    ngOnChanges(): void {
        this.talent_tree = this.i_talent_tree;
    }

    talentChanged(talent: Talent): void {
        this.talent_tree[talent.row_index][talent.column_index] = talent;
        // No idea why a functional approach is not working o.o
        let result = 0;
        for (const row of this.talent_tree) {
            for (const talent of row) {
                result += talent.is_filler ? 0 : talent.points_spend;
            }
        }
        this.points_spend = result;
        this.points_changed.next(result);
    }

    hasDependencyLeftOfIt(talent: Talent): boolean {
        for (let i = 0; i < talent.column_index; ++i) {
            const c_talent = this.talent_tree[talent.row_index][i];
            if (!c_talent.is_filler && !!c_talent.points_to && c_talent.points_to.row_index === talent.row_index
                && c_talent.points_to.column_index >= talent.column_index) {
                return true;
            }
        }
        return false;
    }

    hasArrowPointerLeftToRight(talent: Talent): boolean {
        if (!!talent.points_to && talent.points_to.row_index === talent.row_index && talent.points_to.column_index > talent.column_index)
            return false;

        for (let i = talent.column_index; i < 4; ++i) {
            const c_talent = this.talent_tree[talent.row_index][i];
            if (this.hasDependencyLeftOfIt(c_talent)) {
                return true;
            }
        }

        return false;
    }

    hasDependencyTopOfIt(talent: Talent): boolean {
        for (let i = 0; i < talent.row_index; ++i) {
            const c_talent = this.talent_tree[i][talent.column_index];
            if (!c_talent.is_filler && !!c_talent.points_to && c_talent.points_to.row_index >= talent.row_index
                && c_talent.points_to.column_index === talent.column_index) {
                return true;
            }
        }
        return false;
    }

    hasArrowPointerTopToBottom(talent: Talent): boolean {
        if (!talent.is_filler && !!talent.points_to && talent.points_to.row_index == talent.row_index + 1)
            return true;

        if (!this.hasDependencyTopOfIt(talent))
            return false;

        let dependency = this.findDependency(talent);
        if (!dependency)
            return false;

        return dependency.points_to.row_index === talent.row_index && dependency.points_to.column_index === talent.column_index;
    }

    findDependency(talent: Talent): Talent {
        if (this.hasDependencyLeftOfIt(talent)) {
            if (talent.is_filler && talent.column_index < 3)
                return this.findDependency(this.talent_tree[talent.row_index][talent.column_index + 1]);
            for (let i = 0; i < talent.column_index; ++i) {
                let current_talent = this.talent_tree[talent.row_index][i];
                if (!!current_talent.points_to && current_talent.points_to.row_index === talent.row_index && current_talent.points_to.column_index === talent.column_index)
                    return current_talent;
            }
        } else if (this.hasDependencyTopOfIt(talent)) {
            if (talent.is_filler && talent.row_index < this.talent_tree.length - 1)
                return this.findDependency(this.talent_tree[talent.row_index + 1][talent.column_index]);
            for (let i = 0; i < talent.row_index; ++i) {
                let current_talent = this.talent_tree[i][talent.column_index];
                if (!!current_talent.points_to && current_talent.points_to.row_index === talent.row_index && current_talent.points_to.column_index === talent.column_index)
                    return current_talent;
            }
        }
        return undefined;
    }

    canSpendPoints(talent: Talent): boolean {
        return this.points_spend >= talent.row_index * 5 && !this.isGrayedOut(talent);
    }

    canRetrievePoints(talent: Talent): boolean {
        let max_row_with_points = 0;
        for (let i = 0; i < this.talent_tree.length; ++i) {
            for (let j = 0; j < 4; ++j) {
                const c_talent = this.talent_tree[i][j];
                if (!!c_talent && !c_talent.is_filler && c_talent.points_spend > 0) {
                    max_row_with_points = i;
                }
            }
        }

        let points_spend_this_row_and_below = 0;
        for (let i = 0; i <= talent.row_index; ++i) {
            for (let j = 0; j < 4; ++j) {
                const c_talent = this.talent_tree[i][j];
                if (!c_talent.is_filler && c_talent.points_spend > 0) {
                    points_spend_this_row_and_below += c_talent.points_spend;
                }
            }
        }

        return (max_row_with_points === talent.row_index
            || max_row_with_points === talent.row_index - 1 || points_spend_this_row_and_below > (talent.row_index + 1) * 5)
            && (!talent.points_to || this.talent_tree[talent.points_to.row_index][talent.points_to.column_index].points_spend === 0);
    }

    isGrayedOut(talent: Talent): boolean {
        const dependency = this.findDependency(talent);
        return this.points_spend < talent.row_index * 5 || (!!dependency && dependency.points_spend < dependency.max_points);
    }

    is_arrow_golden(talent: Talent): boolean {
        if (!talent)
            return false;

        if (!!talent.points_to) {
            return !this.isGrayedOut(this.talent_tree[talent.points_to.row_index][talent.points_to.column_index]);
        }
        let dependency = this.findDependency(talent);
        if (!dependency)
            return false;
        return !this.isGrayedOut(this.talent_tree[dependency.points_to.row_index][dependency.points_to.column_index]);
    }

    resetTalents(): void {
        for (let i = 0; i < this.talent_tree.length; ++i)
            for (let j = 0; j < 4; ++j)
                if (!this.talent_tree[i][j].is_filler)
                    this.talent_tree[i][j].points_spend = 0;
        this.points_spend = 0;
        this.points_changed.next(0);
    }
}
