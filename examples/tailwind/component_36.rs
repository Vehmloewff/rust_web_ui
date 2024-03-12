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
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingY(96), Screen::Small(&[Style::PaddingX(24)]), Screen::Small(&[Style::PaddingY(128)]), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("isolate"), NoStyle::Noop("overflow-hidden"), Style::Color(Color::Fg(100)), Style::PaddingX(24), Style::PaddingTop(64), NoStyle::Noop("shadow-2xl"), Screen::Small(&[NoStyle::Noop("rounded-3xl")]), Screen::Small(&[Style::PaddingX(64)]), Screen::Medium(&[Style::PaddingTop(96)]), Screen::Large(&[Style::Flex]), Screen::Large(&[NoStyle::Noop("gap-x-20")]), Screen::Large(&[Style::PaddingX(96)]), Screen::Large(&[Style::PaddingTop(0)])]);
					
					props.child("1", Icon).run(|props| {
						props.style(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-1/2"), NoStyle::Noop("top-1/2"), NoStyle::Noop("-z-10"), NoStyle::Noop("h-[64rem]"), NoStyle::Noop("w-[64rem]"), NoStyle::Noop("-translate-y-1/2"), NoStyle::NoopGroup("[mask-image", NoStyle::Noop("radial-gradient(closest-side,white,transparent)]")), Screen::Small(&[NoStyle::Noop("left-full")]), Screen::Small(&[NoStyle::Noop("-ml-80")]), Screen::Large(&[NoStyle::Noop("left-1/2")]), Screen::Large(&[Style::MarginLeft(0)]), Screen::Large(&[NoStyle::Noop("-translate-x-1/2")]), Screen::Large(&[NoStyle::Noop("translate-y-0")])]);
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-md"), NoStyle::Noop("text-center"), Screen::Large(&[Style::MarginX(0)]), Screen::Large(&[NoStyle::Noop("flex-auto")]), Screen::Large(&[Style::PaddingY(128)]), Screen::Large(&[NoStyle::Noop("text-left")])]);
						
						props.child("1", Dynamic).run("h2", |props| {
							props.styles(&[NoStyle::Noop("text-3xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), NoStyle::Noop("text-white"), Screen::Small(&[NoStyle::Noop("text-4xl")])]);
							
							props.child("0", Label).run(|props| props.set_text("Boost your productivity."));
							props.child("1", Dynamic).run("br", |props| {
								
							});
							props.child("2", Label).run(|props| props.set_text("Start using our app today."));
						});
						props.child("3", Dynamic).run("p", |props| {
							props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-lg"), NoStyle::Noop("leading-8"), Style::TextColor(Color::Fg(33))]);
							
							props.child("0", Label).run(|props| props.set_text("Ac euismod vel sit maecenas id pellentesque eu sed consectetur. Malesuada adipiscing sagittis vel nulla."));
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(40), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("gap-x-6"), Screen::Large(&[NoStyle::Noop("justify-start")])]);
							
							props.child("1", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::PaddingX(14), Style::PaddingY(10), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), Action::Hover(&[Style::Color(Color::Fg(11))]), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-offset-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-white"))]);
								
								props.child("0", Label).run(|props| props.set_text("Get started"));
							});
							props.child("3", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), NoStyle::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.set_text("Learn more"));
								props.child("1", Dynamic).run("span", |props| {
									props.set_attribute("aria-hidden", "true");
									
									props.child("0", Label).run(|props| props.set_text("→"));
								});
							});
						});
					});
					props.child("5", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("relative"), Style::MarginTop(64), Style::Width(320), Screen::Large(&[Style::MarginTop(32)])]);
						
						props.child("1", Dynamic).run("img", |props| {
							props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-0"), NoStyle::Noop("top-0"), NoStyle::Noop("w-[57rem]"), NoStyle::Noop("max-w-none"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white/5"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-white/10")]);
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
