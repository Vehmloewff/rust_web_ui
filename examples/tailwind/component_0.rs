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
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Style::Flex, NoStyle::Noop("min-h-[840px]"), NoStyle::Noop("flex-col"), NoStyle::Noop("bg-white")]);
			
			// 
			//   This example requires updating your template:
			// 
			//   ```
			//   <html class="h-full bg-white">
			//   <body class="h-full">
			//   ```
			//   
			props.child("3", Dynamic).run("div", |props| {
				props.styles(&[Style::Flex, NoStyle::Noop("min-h-full"), NoStyle::Noop("flex-1"), NoStyle::Noop("flex-col"), Style::JustifyCenter, Style::PaddingX(24), Style::PaddingY(48), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[Screen::Small(&[NoStyle::Noop("mx-auto")]), Screen::Small(&[NoStyle::Noop("w-full")]), Screen::Small(&[NoStyle::Noop("max-w-sm")])]);
					
					props.child("1", Dynamic).run("img", |props| {
						props.styles(&[NoStyle::Noop("mx-auto"), Style::Width(40), NoStyle::Noop("w-auto")]);
						props.set_attribute("src", "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600");
						props.set_attribute("alt", "Your Company");
					});
					props.child("3", Dynamic).run("h2", |props| {
						props.styles(&[Style::MarginTop(40), NoStyle::Noop("text-center"), NoStyle::Noop("text-2xl"), Style::FontBold, NoStyle::Noop("leading-9"), NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
						
						props.child("0", Label).run(|props| props.set_text("Sign in to your account"));
					});
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[Style::MarginTop(40), Screen::Small(&[NoStyle::Noop("mx-auto")]), Screen::Small(&[NoStyle::Noop("w-full")]), Screen::Small(&[NoStyle::Noop("max-w-sm")])]);
					
					props.child("1", Dynamic).run("form", |props| {
						props.styles(&[Style::SpaceY(24)]);
						props.set_attribute("action", "#");
						props.set_attribute("method", "POST");
						
						props.child("1", Dynamic).run("div", |props| {
							props.child("1", Dynamic).run("label", |props| {
								props.set_attribute("for", "email");
								props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Email address"));
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(8)]);
								
								props.child("1", Dynamic).run("input", |props| {
									props.set_attribute("id", "email");
									props.set_attribute("name", "email");
									props.set_attribute("type", "email");
									props.set_attribute("autocomplete", "email");
									props.set_attribute("required", "");
									props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingY(6), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
								});
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween]);
								
								props.child("1", Dynamic).run("label", |props| {
									props.set_attribute("for", "password");
									props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Password"));
								});
								props.child("3", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("text-sm")]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::FontSemibold, NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("text-indigo-500")])]);
										
										props.child("0", Label).run(|props| props.set_text("Forgot password?"));
									});
								});
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(8)]);
								
								props.child("1", Dynamic).run("input", |props| {
									props.set_attribute("id", "password");
									props.set_attribute("name", "password");
									props.set_attribute("type", "password");
									props.set_attribute("autocomplete", "current-password");
									props.set_attribute("required", "");
									props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingY(6), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
								});
							});
						});
						props.child("5", Dynamic).run("div", |props| {
							props.child("1", Dynamic).run("button", |props| {
								props.set_attribute("type", "submit");
								props.styles(&[Style::Flex, NoStyle::Noop("w-full"), Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-indigo-600"), Style::PaddingX(12), Style::PaddingY(6), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), NoStyle::Noop("text-white"), NoStyle::Noop("shadow-sm"), Action::Hover(&[NoStyle::Noop("bg-indigo-500")]), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-offset-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-indigo-600"))]);
								
								props.child("0", Label).run(|props| props.set_text("Sign in"));
							});
						});
					});
					props.child("3", Dynamic).run("p", |props| {
						props.styles(&[Style::MarginTop(40), NoStyle::Noop("text-center"), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
						
						props.child("0", Label).run(|props| props.set_text("Not a member?"));
						//  space 
						props.child("3", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::FontSemibold, NoStyle::Noop("leading-6"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("text-indigo-500")])]);
							
							props.child("0", Label).run(|props| props.set_text("Start a 14 day free trial"));
						});
					});
				});
			});
		});
	}
}
