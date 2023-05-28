'use client';
import React, { useContext, useState } from "react";

const Pixel = ({ x, y, color }) => {

  return (
    <>
      <div
        className="pixel"
        style={{
          boxShadow: `${x}rem ${y}rem 0 -0.05rem ${color}`,
        }}
      />
    </>
  );
};

export default Pixel;
