
export const num_range = (start, stop) => {
    return Array.from({ length: (stop - start) + 1}, (_, i) => start + i )
};

export const XY = (x,y) => {
    return {x : x, y: y}
};