// 表单数据类型定义

export interface RegisterFormData {
	username: string;
	email: string;
	password: string;
	full_name?: string;
}

export interface LoginFormData {
	username: string;
	password: string;
}

export interface UpdateUserFormData {
	full_name?: string;
	email?: string;
}

export interface ChangePasswordFormData {
	old_password: string;
	new_password: string;
}

export interface UpdateUserRoleFormData {
	role: string;
}
