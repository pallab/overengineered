
import { num_range, XY } from "./utils.js";

export const getLetterPixels = (c) => {
    switch (c) {
        case 'H': return h();
        case 'E': return e();
        case 'L': return l();
        case 'O': return o();
        case 'W': return w();
        case 'R': return r();
        case 'D': return d();
        case '!': return exclm();
        default:
            return random_pattrn();
    }
};

const h = () => {
    let a = num_range(2, 8).flatMap((y) => {
        return [XY(2, y), XY(6, y)]
    });

    let b = [XY(3, 5), XY(4, 5), XY(5, 5)];
    return a.concat(b)
}
const e = () => {
    let a = num_range(2, 8).flatMap((y) => {
        return [XY(2, y), XY(6, y)]
    });

    let b = [XY(3, 5), XY(4, 5), XY(5, 5)];
    return a.concat(b)
}
const l = () => {
    let a = num_range(2, 8).flatMap((y) => {
        return [XY(2, y), XY(6, y)]
    });

    let b = [XY(3, 5), XY(4, 5), XY(5, 5)];
    return a.concat(b)
}

const o = () => {
    let a = num_range(2, 8).flatMap((y) => {
        return [XY(2, y), XY(6, y)]
    });

    let b = [XY(3, 5), XY(4, 5), XY(5, 5)];
    return a.concat(b)
}
const w = () => {
    let a = num_range(2, 8).flatMap((y) => {
        return [XY(2, y), XY(6, y)]
    });

    let b = [XY(3, 5), XY(4, 5), XY(5, 5)];
    return a.concat(b)
}
const r = () => {
    let a = num_range(2, 8).flatMap((y) => {
        return [XY(2, y), XY(6, y)]
    });

    let b = [XY(3, 5), XY(4, 5), XY(5, 5)];
    return a.concat(b)
}
const d = () => {
    let a = num_range(2, 8).flatMap((y) => {
        return [XY(2, y), XY(6, y)]
    });

    let b = [XY(3, 5), XY(4, 5), XY(5, 5)];
    return a.concat(b)
}
const exclm = () => {
    let a = num_range(2, 8).flatMap((y) => {
        return [XY(2, y), XY(6, y)]
    });

    let b = [XY(3, 5), XY(4, 5), XY(5, 5)];
    return a.concat(b)
}

const random_pattrn = () => {
    return [XY(1, 1), XY(2, 2), XY(5, 5)]
}
