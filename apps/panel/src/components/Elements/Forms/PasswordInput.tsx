'use client'
import * as React from 'react';
import { TextInputProps } from '@components/Elements/Forms/TextInput';

interface PasswordInputProps extends TextInputProps {
    valid?: boolean;
}

const PasswordInput: React.FC<PasswordInputProps> = ({ placeholder, value, className, valid, ...props }) => {
    return (
        <div>
            <input
                className={`appearance-none rounded-md px-3 bg-neutral-800 placeholder:text-neutral-600 outline-none focus:border-blue-400 ${className} ${!valid && 'border-2 border-red-500'}`}
                type="password"
                placeholder={placeholder}
                {...props}
            />
        </div>
    );
};

export default PasswordInput;