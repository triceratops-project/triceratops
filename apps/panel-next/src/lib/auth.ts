'use client'
export function isUserAuthenticated(): boolean {
    return !!localStorage.getItem('authorization'); // FIXME: this erroring
}

export function logoutUser() {
    localStorage.removeItem('authorization');
    localStorage.removeItem('user');
}

export function getAuthToken(): string | null {
    return localStorage.getItem('authorization');
}

export function getUserInfo() {
    const userInfo = localStorage.getItem('user') || '';
    return JSON.parse(userInfo);
}
