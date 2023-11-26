'use client'
import * as React from 'react';

export interface TextInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
}

const TextInput: React.FC<TextInputProps> = ({ placeholder, value, className, ...props }) => {
    return (
        <div>
            <input
                className={`appearance-none rounded-md px-3 bg-neutral-800 placeholder:text-neutral-600 ${className}`}
                type="text"
                placeholder={placeholder}
                {...props}
            />
        </div>
    );
};

export default TextInput;