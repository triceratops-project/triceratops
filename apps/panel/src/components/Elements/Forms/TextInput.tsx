'use client'
import * as React from 'react';

export interface TextInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
    valid?: boolean;
}

const TextInput: React.FC<TextInputProps> = ({ placeholder, value, className, valid, ...props }) => {
    return (
        <div>
            <input
                className={`appearance-none rounded-md px-3 bg-neutral-800 placeholder:text-neutral-600 focus:border-blue-400 outline-none ${className} ${!valid && 'border-2 border-red-500'}`}
                type="text"
                placeholder={placeholder}
                {...props}
            />
        </div>
    );
};

export default TextInput;