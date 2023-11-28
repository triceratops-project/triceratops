'use client'
import * as React from 'react';

interface SubmitProps extends React.InputHTMLAttributes<HTMLInputElement> {
    value: string;
}

const Submit: React.FC<SubmitProps> = ({ value, className, ...props }) => {
    return (
        <div>
            <input
                className={`appearance-none rounded-md bg-pink-500 hover:bg-pink-400 transition cursor-pointer ${className}`}
                type="submit"
                value={value}
                {...props}
            />
        </div>
    );
};

export default Submit; 