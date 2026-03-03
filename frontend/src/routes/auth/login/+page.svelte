<script lang="ts">
	import LoginForm from '$lib/components/auth/LoginForm.svelte';
</script>

<div class="min-h-screen grid lg:grid-cols-2">
	<!-- Left Panel - Login Form -->
	<div class="flex items-center justify-center px-4 py-12 sm:px-6 lg:px-8 bg-white">
		<div class="max-w-md w-full space-y-8 animate-fadeIn">
			<div class="text-center">
				<div class="inline-flex items-center justify-center w-16 h-16 rounded-2xl gradient-primary text-white text-3xl mb-4 shadow-lg">
					💬
				</div>
				<h1 class="text-4xl font-bold text-gray-900 mb-2">欢迎回来</h1>
				<p class="text-gray-600 text-lg">登录继续使用聊天服务</p>
			</div>

			<div class="bg-gray-50 p-8 rounded-2xl shadow-sm border border-gray-100">
				<LoginForm />
			</div>

			<p class="text-center text-sm text-gray-600">
				还没有账号？
				<a href="/auth/register" class="text-blue-600 hover:text-blue-700 font-semibold">立即注册</a>
			</p>
		</div>
	</div>

	<!-- Right Panel - Decorative -->
	<div class="hidden lg:flex items-center justify-center gradient-primary text-white p-12">
		<div class="max-w-md space-y-8 animate-slideInFromRight">
			<div>
				<h2 class="text-4xl font-bold mb-4">实时聊天，随时随地</h2>
				<p class="text-blue-100 text-lg leading-relaxed">
					加入我们的聊天平台，与朋友和团队保持联系，享受流畅的实时通信体验。
				</p>
			</div>

			<div class="space-y-6">
				<div class="flex items-start gap-4 p-6 glass rounded-xl">
					<div class="flex-shrink-0 w-12 h-12 rounded-lg bg-white/20 flex items-center justify-center text-2xl">
						⚡
					</div>
					<div>
						<h3 class="font-bold text-lg mb-1">实时消息</h3>
						<p class="text-blue-100 text-sm">消息即时送达，无需等待刷新</p>
					</div>
				</div>

				<div class="flex items-start gap-4 p-6 glass rounded-xl">
					<div class="flex-shrink-0 w-12 h-12 rounded-lg bg-white/20 flex items-center justify-center text-2xl">
						🔒
					</div>
					<div>
						<h3 class="font-bold text-lg mb-1">安全可靠</h3>
						<p class="text-blue-100 text-sm">端到端加密，保护您的隐私</p>
					</div>
				</div>

				<div class="flex items-start gap-4 p-6 glass rounded-xl">
					<div class="flex-shrink-0 w-12 h-12 rounded-lg bg-white/20 flex items-center justify-center text-2xl">
						👥
					</div>
					<div>
						<h3 class="font-bold text-lg mb-1">多人协作</h3>
						<p class="text-blue-100 text-sm">创建房间，邀请成员，高效沟通</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
