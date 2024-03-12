use rust_web_ui::*;

pub struct Example40;

pub struct Example40Props {}

impl Default for Example40Props {
	fn default() -> Example40Props {
		Example40Props { }
	}
}

impl Widget<'_> for Example40 {
	type Props = Example40Props;

	fn render(mut ctx: Ctx<'_>, props: Example40Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("isolate"), NoStyle::Noop("overflow-hidden"), Style::Color(Color::Fg(100)), Style::PaddingY(64), Screen::Small(&[Style::PaddingY(96)]), Screen::Large(&[Style::PaddingY(128)])]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(24), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("grid"), NoStyle::Noop("max-w-2xl"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-16"), Screen::Large(&[NoStyle::Noop("max-w-none")]), Screen::Large(&[NoStyle::Noop("grid-cols-2")])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("max-w-xl"), Screen::Large(&[NoStyle::Noop("max-w-lg")])]);
						
						props.child("1", Dynamic).run("h2", |props| {
							props.styles(&[NoStyle::Noop("text-3xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), NoStyle::Noop("text-white"), Screen::Small(&[NoStyle::Noop("text-4xl")])]);
							
							props.child("0", Label).run(|props| props.set_text("Subscribe to our newsletter."));
						});
						props.child("3", Dynamic).run("p", |props| {
							props.styles(&[Style::MarginTop(16), NoStyle::Noop("text-lg"), NoStyle::Noop("leading-8"), Style::TextColor(Color::Fg(33))]);
							
							props.child("0", Label).run(|props| props.set_text("Nostrud amet eu ullamco nisi aute in ad minim nostrud adipisicing velit quis. Duis tempor incididunt dolore."));
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(24), Style::Flex, NoStyle::Noop("max-w-md"), NoStyle::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("label", |props| {
								props.set_attribute("for", "email-address");
								props.styles(&[NoStyle::Noop("sr-only")]);
								
								props.child("0", Label).run(|props| props.set_text("Email address"));
							});
							props.child("3", Dynamic).run("input", |props| {
								props.set_attribute("id", "email-address");
								props.set_attribute("name", "email");
								props.set_attribute("type", "email");
								props.set_attribute("autocomplete", "email");
								props.set_attribute("required", "");
								props.styles(&[NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-auto"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), NoStyle::Noop("bg-white/5"), Style::PaddingX(14), Style::PaddingY(8), NoStyle::Noop("text-white"), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-white/10"), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-500")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
								props.set_attribute("placeholder", "Enter your email");
							});
							props.child("5", Dynamic).run("button", |props| {
								props.set_attribute("type", "submit");
								props.styles(&[NoStyle::Noop("flex-none"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-indigo-500"), Style::PaddingX(14), Style::PaddingY(10), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("text-white"), NoStyle::Noop("shadow-sm"), Action::Hover(&[NoStyle::Noop("bg-indigo-400")]), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-offset-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-indigo-500"))]);
								
								props.child("0", Label).run(|props| props.set_text("Subscribe"));
							});
						});
					});
					props.child("3", Dynamic).run("dl", |props| {
						props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-10"), Screen::Small(&[NoStyle::Noop("grid-cols-2")]), Screen::Large(&[Style::PaddingTop(8)])]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("flex-col"), NoStyle::Noop("items-start")]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white/5"), Style::Padding(8), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-white/10")]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[Style::Width(24), Style::Width(24), NoStyle::Noop("text-white")]);
								});
							});
							props.child("3", Dynamic).run("dt", |props| {
								props.styles(&[Style::MarginTop(16), Style::FontSemibold, NoStyle::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.set_text("Weekly articles"));
							});
							props.child("5", Dynamic).run("dd", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(44))]);
								
								props.child("0", Label).run(|props| props.set_text("Non laboris consequat cupidatat laborum magna. Eiusmod non irure cupidatat duis commodo amet."));
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("flex-col"), NoStyle::Noop("items-start")]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white/5"), Style::Padding(8), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-white/10")]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[Style::Width(24), Style::Width(24), NoStyle::Noop("text-white")]);
								});
							});
							props.child("3", Dynamic).run("dt", |props| {
								props.styles(&[Style::MarginTop(16), Style::FontSemibold, NoStyle::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.set_text("No spam"));
							});
							props.child("5", Dynamic).run("dd", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(44))]);
								
								props.child("0", Label).run(|props| props.set_text("Officia excepteur ullamco ut sint duis proident non adipisicing. Voluptate incididunt anim."));
							});
						});
					});
				});
			});
			props.child("3", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-1/2"), NoStyle::Noop("top-0"), NoStyle::Noop("-z-10"), NoStyle::Noop("-translate-x-1/2"), NoStyle::Noop("blur-3xl"), Screen::ExtraLarge(1, &[NoStyle::Noop("-top-6")])]);
				props.set_attribute("aria-hidden", "true");
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("aspect-[1155/678]"), NoStyle::Noop("w-[72.1875rem]"), NoStyle::Noop("bg-gradient-to-tr"), NoStyle::Noop("from-[#ff80b5]"), NoStyle::Noop("to-[#9089fc]"), NoStyle::Noop("opacity-30")]);
					props.set_attribute("style", "clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)");
				});
			});
		});
	}
}
