'use client'
import * as React from 'react';

interface TextInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
    label: string;
}

const TextInput: React.FC<TextInputProps> = ({ label, placeholder, value, className, ...props }) => {
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