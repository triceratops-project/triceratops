'use client'
import * as React from 'react';
import { TextInputProps } from '@components/Elements/Forms/TextInput';

interface PasswordInputProps extends TextInputProps {
}

const PasswordInput: React.FC<PasswordInputProps> = ({ placeholder, value, className, ...props }) => {
    return (
        <div>
            <input
                className={`appearance-none rounded-md px-3 bg-neutral-800 placeholder:text-neutral-600 ${className}`}
                type="password"
                placeholder={placeholder}
                {...props}
            />
        </div>
    );
};

export default PasswordInput;