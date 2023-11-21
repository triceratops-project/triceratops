'use client'
import * as React from 'react';

interface TextInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
    label: string;
}

const TextInput: React.FC<TextInputProps> = ({ label, value, className, ...props }) => {
    return (
        <div>
            <input
                className={`appearance-none rounded-md bg-neutral-800 ${className}`}
                type="text"
                {...props}
            />
        </div>
    );
};

export default TextInput;