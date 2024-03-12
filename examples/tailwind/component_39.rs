use rust_web_ui::*;

pub struct Example39;

pub struct Example39Props {}

impl Default for Example39Props {
	fn default() -> Example39Props {
		Example39Props { }
	}
}

impl Widget<'_> for Example39 {
	type Props = Example39Props;

	fn render(mut ctx: Ctx<'_>, props: Example39Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white")]);
			props.set_attribute("style", "min-height: 16rem");
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("relative"), Style::Noop("isolate"), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-6"), Style::Noop("overflow-hidden"), Style::Color(Color::Fg(6)), Style::PaddingX(Size::Exact(24)), Style::PaddingY(Size::Exact(10)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(14))]), Style::OnScreen(Screen::Small, &[Style::NoopGroup("before", &[Style::Noop("flex-1")])])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("absolute"), Style::Noop("left-[max(-7rem,calc(50%-52rem))]"), Style::Noop("top-1/2"), Style::Noop("-z-10"), Style::Noop("-translate-y-1/2"), Style::Noop("transform-gpu"), Style::Noop("blur-2xl")]);
					props.set_attribute("aria-hidden", "true");
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("aspect-[577/310]"), Style::Noop("w-[36.0625rem]"), Style::Noop("bg-gradient-to-r"), Style::Noop("from-[#ff80b5]"), Style::Noop("to-[#9089fc]"), Style::Noop("opacity-30")]);
						props.set_attribute("style", "clip-path: polygon(74.8% 41.9%, 97.2% 73.2%, 100% 34.9%, 92.5% 0.4%, 87.5% 0%, 75% 28.6%, 58.5% 54.6%, 50.1% 56.8%, 46.9% 44%, 48.3% 17.4%, 24.7% 53.9%, 0% 27.9%, 11.9% 74.2%, 24.9% 54.1%, 68.6% 100%, 74.8% 41.9%)");
					});
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("absolute"), Style::Noop("left-[max(45rem,calc(50%+8rem))]"), Style::Noop("top-1/2"), Style::Noop("-z-10"), Style::Noop("-translate-y-1/2"), Style::Noop("transform-gpu"), Style::Noop("blur-2xl")]);
					props.set_attribute("aria-hidden", "true");
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("aspect-[577/310]"), Style::Noop("w-[36.0625rem]"), Style::Noop("bg-gradient-to-r"), Style::Noop("from-[#ff80b5]"), Style::Noop("to-[#9089fc]"), Style::Noop("opacity-30")]);
						props.set_attribute("style", "clip-path: polygon(74.8% 41.9%, 97.2% 73.2%, 100% 34.9%, 92.5% 0.4%, 87.5% 0%, 75% 28.6%, 58.5% 54.6%, 50.1% 56.8%, 46.9% 44%, 48.3% 17.4%, 24.7% 53.9%, 0% 27.9%, 11.9% 74.2%, 24.9% 54.1%, 68.6% 100%, 74.8% 41.9%)");
					});
				});
				props.child("5", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Flex, Style::Noop("flex-wrap"), Style::ItemsCenter, Style::Noop("gap-x-4"), Style::Noop("gap-y-2")]);
					
					props.child("1", Dynamic).run("p", |mut props| {
						props.styles(&[Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
						
						props.child("1", Dynamic).run("strong", |mut props| {
							props.styles(&[Style::FontSemibold]);
							
							props.child("0", Label).run(|props| props.text("GeneriCon 2023"));
						});
						props.child("2", Icon).run(|mut props| {
							props.style(&[Style::MarginX(Size::Exact(8)), Style::Noop("inline"), Style::Width(Size::Exact(2)), Style::Width(Size::Exact(2)), Style::Noop("fill-current")]);
						});
						props.child("3", Label).run(|props| props.text("Join us in Denver from June 7 – 9 to see what’s coming next."));
					});
					props.child("3", Dynamic).run("a", |mut props| {
						props.set_attribute("href", "#");
						props.styles(&[Style::Noop("flex-none"), Style::Noop("rounded-full"), Style::Color(Color::Fg(100)), Style::PaddingX(Size::Exact(14)), Style::PaddingY(Size::Exact(4)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("text-white"), Style::Noop("shadow-sm"), Style::OnHover(&[Style::Color(Color::Fg(78))]), Style::NoopGroup("focus-visible", &[Style::Noop("outline")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-offset-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-gray-900")])]);
						
						props.child("0", Label).run(|props| props.text("Register now"));
						props.child("1", Dynamic).run("span", |mut props| {
							props.set_attribute("aria-hidden", "true");
							
							props.child("0", Label).run(|props| props.text("→"));
						});
					});
				});
				props.child("7", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Flex, Style::Noop("flex-1"), Style::Noop("justify-end")]);
					
					props.child("1", Dynamic).run("button", |mut props| {
						props.set_attribute("type", "button");
						props.styles(&[Style::Noop("-m-3"), Style::Padding(Size::Exact(12)), Style::NoopGroup("focus-visible", &[Style::Noop("outline-offset-[-4px]")])]);
						
						props.child("1", Dynamic).run("span", |mut props| {
							props.styles(&[Style::Noop("sr-only")]);
							
							props.child("0", Label).run(|props| props.text("Dismiss"));
						});
						props.child("3", Icon).run(|mut props| {
							props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::TextColor(Color::Fg(100))]);
						});
					});
				});
			});
		});
	}
}
