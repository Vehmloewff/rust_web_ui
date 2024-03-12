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
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white")]);
			props.set_attribute("style", "min-height: 16rem");
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("isolate"), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-6"), NoStyle::Noop("overflow-hidden"), Style::Color(Color::Fg(6)), Style::PaddingX(24), Style::PaddingY(10), Screen::Small(&[Style::PaddingX(14)]), Screen::Small(&[NoStyle::NoopGroup("before", NoStyle::Noop("flex-1"))])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-[max(-7rem,calc(50%-52rem))]"), NoStyle::Noop("top-1/2"), NoStyle::Noop("-z-10"), NoStyle::Noop("-translate-y-1/2"), NoStyle::Noop("transform-gpu"), NoStyle::Noop("blur-2xl")]);
					props.set_attribute("aria-hidden", "true");
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("aspect-[577/310]"), NoStyle::Noop("w-[36.0625rem]"), NoStyle::Noop("bg-gradient-to-r"), NoStyle::Noop("from-[#ff80b5]"), NoStyle::Noop("to-[#9089fc]"), NoStyle::Noop("opacity-30")]);
						props.set_attribute("style", "clip-path: polygon(74.8% 41.9%, 97.2% 73.2%, 100% 34.9%, 92.5% 0.4%, 87.5% 0%, 75% 28.6%, 58.5% 54.6%, 50.1% 56.8%, 46.9% 44%, 48.3% 17.4%, 24.7% 53.9%, 0% 27.9%, 11.9% 74.2%, 24.9% 54.1%, 68.6% 100%, 74.8% 41.9%)");
					});
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-[max(45rem,calc(50%+8rem))]"), NoStyle::Noop("top-1/2"), NoStyle::Noop("-z-10"), NoStyle::Noop("-translate-y-1/2"), NoStyle::Noop("transform-gpu"), NoStyle::Noop("blur-2xl")]);
					props.set_attribute("aria-hidden", "true");
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("aspect-[577/310]"), NoStyle::Noop("w-[36.0625rem]"), NoStyle::Noop("bg-gradient-to-r"), NoStyle::Noop("from-[#ff80b5]"), NoStyle::Noop("to-[#9089fc]"), NoStyle::Noop("opacity-30")]);
						props.set_attribute("style", "clip-path: polygon(74.8% 41.9%, 97.2% 73.2%, 100% 34.9%, 92.5% 0.4%, 87.5% 0%, 75% 28.6%, 58.5% 54.6%, 50.1% 56.8%, 46.9% 44%, 48.3% 17.4%, 24.7% 53.9%, 0% 27.9%, 11.9% 74.2%, 24.9% 54.1%, 68.6% 100%, 74.8% 41.9%)");
					});
				});
				props.child("5", Dynamic).run("div", |props| {
					props.styles(&[Style::Flex, NoStyle::Noop("flex-wrap"), Style::ItemsCenter, NoStyle::Noop("gap-x-4"), NoStyle::Noop("gap-y-2")]);
					
					props.child("1", Dynamic).run("p", |props| {
						props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
						
						props.child("1", Dynamic).run("strong", |props| {
							props.styles(&[Style::FontSemibold]);
							
							props.child("0", Label).run(|props| props.set_text("GeneriCon 2023"));
						});
						props.child("2", Icon).run(|props| {
							props.style(&[Style::MarginX(8), NoStyle::Noop("inline"), Style::Width(2), Style::Width(2), NoStyle::Noop("fill-current")]);
						});
						props.child("3", Label).run(|props| props.set_text("Join us in Denver from June 7 – 9 to see what’s coming next."));
					});
					props.child("3", Dynamic).run("a", |props| {
						props.set_attribute("href", "#");
						props.styles(&[NoStyle::Noop("flex-none"), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(100)), Style::PaddingX(14), Style::PaddingY(4), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("text-white"), NoStyle::Noop("shadow-sm"), Action::Hover(&[Style::Color(Color::Fg(78))]), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-offset-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-gray-900"))]);
						
						props.child("0", Label).run(|props| props.set_text("Register now"));
						props.child("1", Dynamic).run("span", |props| {
							props.set_attribute("aria-hidden", "true");
							
							props.child("0", Label).run(|props| props.set_text("→"));
						});
					});
				});
				props.child("7", Dynamic).run("div", |props| {
					props.styles(&[Style::Flex, NoStyle::Noop("flex-1"), NoStyle::Noop("justify-end")]);
					
					props.child("1", Dynamic).run("button", |props| {
						props.set_attribute("type", "button");
						props.styles(&[NoStyle::Noop("-m-3"), Style::Padding(12), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-offset-[-4px]"))]);
						
						props.child("1", Dynamic).run("span", |props| {
							props.styles(&[NoStyle::Noop("sr-only")]);
							
							props.child("0", Label).run(|props| props.set_text("Dismiss"));
						});
						props.child("3", Icon).run(|props| {
							props.style(&[Style::Width(20), Style::Width(20), Style::TextColor(Color::Fg(100))]);
						});
					});
				});
			});
		});
	}
}
