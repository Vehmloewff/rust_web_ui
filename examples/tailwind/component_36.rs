use rust_web_ui::*;

pub struct Example36;

pub struct Example36Props {}

impl Default for Example36Props {
	fn default() -> Example36Props {
		Example36Props { }
	}
}

impl Widget<'_> for Example36 {
	type Props = Example36Props;

	fn render(mut ctx: Ctx<'_>, props: Example36Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))]), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("relative"), Style::Noop("isolate"), Style::Noop("overflow-hidden"), Style::Color(Color::Fg(100)), Style::PaddingX(Size::Exact(24)), Style::PaddingTop(Size::Exact(64)), Style::Noop("shadow-2xl"), Style::OnScreen(Screen::Small, &[Style::Noop("rounded-3xl")]), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(64))]), Style::OnScreen(Screen::Medium, &[Style::PaddingTop(Size::Exact(96))]), Style::OnScreen(Screen::Large, &[Style::Flex]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-x-20")]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(96))]), Style::OnScreen(Screen::Large, &[Style::PaddingTop(Size::Exact(0))])]);
					
					props.child("1", Icon).run(|mut props| {
						props.style(&[Style::Noop("absolute"), Style::Noop("left-1/2"), Style::Noop("top-1/2"), Style::Noop("-z-10"), Style::Noop("h-[64rem]"), Style::Noop("w-[64rem]"), Style::Noop("-translate-y-1/2"), Style::NoopGroup("[mask-image", &[Style::Noop("radial-gradient(closest-side,white,transparent)]")]), Style::OnScreen(Screen::Small, &[Style::Noop("left-full")]), Style::OnScreen(Screen::Small, &[Style::Noop("-ml-80")]), Style::OnScreen(Screen::Large, &[Style::Noop("left-1/2")]), Style::OnScreen(Screen::Large, &[Style::MarginLeft(Size::Exact(0))]), Style::OnScreen(Screen::Large, &[Style::Noop("-translate-x-1/2")]), Style::OnScreen(Screen::Large, &[Style::Noop("translate-y-0")])]);
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-md"), Style::Noop("text-center"), Style::OnScreen(Screen::Large, &[Style::MarginX(Size::Exact(0))]), Style::OnScreen(Screen::Large, &[Style::Noop("flex-auto")]), Style::OnScreen(Screen::Large, &[Style::PaddingY(Size::Exact(128))]), Style::OnScreen(Screen::Large, &[Style::Noop("text-left")])]);
						
						props.child("1", Dynamic).run("h2", |mut props| {
							props.styles(&[Style::Noop("text-3xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::Noop("text-white"), Style::OnScreen(Screen::Small, &[Style::Noop("text-4xl")])]);
							
							props.child("0", Label).run(|props| props.text("Boost your productivity."));
							props.child("1", Dynamic).run("br", |mut props| {
								
							});
							props.child("2", Label).run(|props| props.text("Start using our app today."));
						});
						props.child("3", Dynamic).run("p", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-lg"), Style::Noop("leading-8"), Style::TextColor(Color::Fg(33))]);
							
							props.child("0", Label).run(|props| props.text("Ac euismod vel sit maecenas id pellentesque eu sed consectetur. Malesuada adipiscing sagittis vel nulla."));
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(40)), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("gap-x-6"), Style::OnScreen(Screen::Large, &[Style::Noop("justify-start")])]);
							
							props.child("1", Dynamic).run("a", |mut props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(14)), Style::PaddingY(Size::Exact(10)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::OnHover(&[Style::Color(Color::Fg(11))]), Style::NoopGroup("focus-visible", &[Style::Noop("outline")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-offset-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-white")])]);
								
								props.child("0", Label).run(|props| props.text("Get started"));
							});
							props.child("3", Dynamic).run("a", |mut props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.text("Learn more"));
								props.child("1", Dynamic).run("span", |mut props| {
									props.set_attribute("aria-hidden", "true");
									
									props.child("0", Label).run(|props| props.text("→"));
								});
							});
						});
					});
					props.child("5", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("relative"), Style::MarginTop(Size::Exact(64)), Style::Width(Size::Exact(320)), Style::OnScreen(Screen::Large, &[Style::MarginTop(Size::Exact(32))])]);
						
						props.child("1", Dynamic).run("img", |mut props| {
							props.styles(&[Style::Noop("absolute"), Style::Noop("left-0"), Style::Noop("top-0"), Style::Noop("w-[57rem]"), Style::Noop("max-w-none"), Style::Noop("rounded-md"), Style::Noop("bg-white/5"), Style::Noop("ring-1"), Style::Noop("ring-white/10")]);
							props.set_attribute("src", "https://tailwindui.com/img/component-images/dark-project-app-screenshot.png");
							props.set_attribute("alt", "App screenshot");
							props.set_attribute("width", "1824");
							props.set_attribute("height", "1080");
						});
					});
				});
			});
		});
	}
}
