import { React, useState, useEffect } from "react";
import bR from './images/bR.png';
import bN from './images/bN.png';
import bB from './images/bB.png';
import bQ from './images/bQ.png';
import bK from './images/bK.png';
import bp from './images/bp.png';
import wR from './images/wR.png';
import wN from './images/wN.png';
import wB from './images/wB.png';
import wQ from './images/wQ.png';
import wK from './images/wK.png';
import wp from './images/wp.png';

function Chessboard() {

    // Number of rows
    const n = 8;
    // Number of columns
    const m = 8;

    // Array which will be used to 
    // display the chessboard
    const [chessBoard, setChessBoard] = useState([]);
    const [images, initializeImages] = useState([]);
    const [clickedPiece, setClickedPiece] = useState(null);
    const [isDragging, setIsDragging] = useState(false);
    const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });



    useEffect(() => {

        // Initialize result with an empty array
        const result = [];

        for (let i = 0; i < n; i++) {

            const row = Array.from({ length: m });
            result.push(row);
        }


        setChessBoard(result);
        let images = [
            bR, bN, bB, bQ, bK, bB, bN, bR,
            bp, bp, bp, bp, bp, bp, bp, bp,
            null, null, null, null, null, null, null, null,
            null, null, null, null, null, null, null, null,
            null, null, null, null, null, null, null, null,
            null, null, null, null, null, null, null, null,
            wp, wp, wp, wp, wp, wp, wp, wp,
            wR, wN, wB, wQ, wK, wB, wN, wR
        ];


        const handleMouseMove = (e) => {
            setMousePosition({ x: e.clientX, y: e.clientY });
        };

        initializeImages(images);

        window.addEventListener('mousemove', handleMouseMove);

        return () => {
            window.removeEventListener('mousemove', handleMouseMove);
        };
    }, []);


    function bitBoardToImages(bitBoard) {

    }

    function onDown(e, piece, idx) {
        e.preventDefault();
        if (!piece) {
            return
        }
        setIsDragging(true);
        setClickedPiece({piece, idx});
        images[idx] = null;
    }

    function movePiece(e, idx) {
        if (!clickedPiece || !isDragging) {
            setClickedPiece(null)
            setIsDragging(false)
            return
        }
        if (idx === clickedPiece.idx) {
            if (images[idx] == null) {
                images[idx] = clickedPiece.piece;
            }
            setClickedPiece(null)
            setIsDragging(false)
            return
        }
        images[idx] = clickedPiece.piece;
        images[clickedPiece.idx] = null;
        setIsDragging(false);
        setClickedPiece(null);
        initializeImages(images);


    }

    return (
        <>
            {chessBoard.length > 0 &&
                chessBoard.map((row, rIndex) => (
                    <div className="flex" key={rIndex}>
                        {row.map((_, cIndex) => {
                            const piece = images[rIndex * 8 + cIndex];
                            return (
                                <div
                                    className={`w-[80px] h-[80px] flex items-center justify-center ${(rIndex + cIndex) % 2 === 0
                                        ? "bg-chessGreen" : "bg-white"
                                        }`}
                                    key={cIndex} onMouseUp={(e) => movePiece(e, rIndex * 8 + cIndex)} onMouseDown={(e) => onDown(e, piece, rIndex * 8 + cIndex)}
                                >
                                    {piece ? (
                                        <img
                                            src={piece}
                                            alt={`piece-${rIndex}-${cIndex}`}
                                            className="object-contain"
                                        />
                                    ) : null}
                                </div>
                            );
                        })}
                    </div>
                ))}


            {isDragging && clickedPiece && (
                <img
                    src={clickedPiece.piece}
                    alt={clickedPiece.piece}
                    className="object-contain absolute"
                    style={{
                        left: mousePosition.x - 40,
                        top: mousePosition.y - 40,
                        width: "80px",
                        height: "80px",
                        pointerEvents: "none",
                    }}
                />
            )}
        </>
    );


}

export default Chessboard;