use rust_web_ui::*;

pub struct Example0;

pub struct Example0Props {}

impl Default for Example0Props {
	fn default() -> Example0Props {
		Example0Props { }
	}
}

impl Widget<'_> for Example0 {
	type Props = Example0Props;

	fn render(mut ctx: Ctx<'_>, props: Example0Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Flex, Style::Noop("min-h-[840px]"), Style::Noop("flex-col"), Style::Noop("bg-white")]);
			
			// 
			//   This example requires updating your template:
			// 
			//   ```
			//   <html class="h-full bg-white">
			//   <body class="h-full">
			//   ```
			//   
			props.child("3", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Flex, Style::Noop("min-h-full"), Style::Noop("flex-1"), Style::Noop("flex-col"), Style::JustifyCenter, Style::PaddingX(Size::Exact(24)), Style::PaddingY(Size::Exact(48)), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::OnScreen(Screen::Small, &[Style::Noop("mx-auto")]), Style::OnScreen(Screen::Small, &[Style::Width(Size::Full)]), Style::OnScreen(Screen::Small, &[Style::Noop("max-w-sm")])]);
					
					props.child("1", Dynamic).run("img", |mut props| {
						props.styles(&[Style::Noop("mx-auto"), Style::Width(Size::Exact(40)), Style::Noop("w-auto")]);
						props.set_attribute("src", "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600");
						props.set_attribute("alt", "Your Company");
					});
					props.child("3", Dynamic).run("h2", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(40)), Style::Noop("text-center"), Style::Noop("text-2xl"), Style::FontBold, Style::Noop("leading-9"), Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
						
						props.child("0", Label).run(|props| props.text("Sign in to your account"));
					});
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::MarginTop(Size::Exact(40)), Style::OnScreen(Screen::Small, &[Style::Noop("mx-auto")]), Style::OnScreen(Screen::Small, &[Style::Width(Size::Full)]), Style::OnScreen(Screen::Small, &[Style::Noop("max-w-sm")])]);
					
					props.child("1", Dynamic).run("form", |mut props| {
						props.styles(&[Style::SpaceY(Size::Exact(24))]);
						props.set_attribute("action", "#");
						props.set_attribute("method", "POST");
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.child("1", Dynamic).run("label", |mut props| {
								props.set_attribute("for", "email");
								props.styles(&[Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.text("Email address"));
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8))]);
								
								props.child("1", Dynamic).run("input", |mut props| {
									props.set_attribute("id", "email");
									props.set_attribute("name", "email");
									props.set_attribute("type", "email");
									props.set_attribute("autocomplete", "email");
									props.set_attribute("required", "");
									props.styles(&[Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingY(Size::Exact(6)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
								});
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween]);
								
								props.child("1", Dynamic).run("label", |mut props| {
									props.set_attribute("for", "password");
									props.styles(&[Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.text("Password"));
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("text-sm")]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::FontSemibold, Style::Noop("text-indigo-600"), Style::OnHover(&[Style::Noop("text-indigo-500")])]);
										
										props.child("0", Label).run(|props| props.text("Forgot password?"));
									});
								});
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8))]);
								
								props.child("1", Dynamic).run("input", |mut props| {
									props.set_attribute("id", "password");
									props.set_attribute("name", "password");
									props.set_attribute("type", "password");
									props.set_attribute("autocomplete", "current-password");
									props.set_attribute("required", "");
									props.styles(&[Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingY(Size::Exact(6)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
								});
							});
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.child("1", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "submit");
								props.styles(&[Style::Flex, Style::Width(Size::Full), Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("bg-indigo-600"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(6)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::Noop("text-white"), Style::Noop("shadow-sm"), Style::OnHover(&[Style::Noop("bg-indigo-500")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-offset-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-indigo-600")])]);
								
								props.child("0", Label).run(|props| props.text("Sign in"));
							});
						});
					});
					props.child("3", Dynamic).run("p", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(40)), Style::Noop("text-center"), Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
						
						props.child("0", Label).run(|props| props.text("Not a member?"));
						//  space 
						props.child("3", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::FontSemibold, Style::Noop("leading-6"), Style::Noop("text-indigo-600"), Style::OnHover(&[Style::Noop("text-indigo-500")])]);
							
							props.child("0", Label).run(|props| props.text("Start a 14 day free trial"));
						});
					});
				});
			});
		});
	}
}
