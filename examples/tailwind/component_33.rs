use rust_web_ui::*;

pub struct Example33;

pub struct Example33Props {}

impl Default for Example33Props {
	fn default() -> Example33Props {
		Example33Props { }
	}
}

impl Widget<'_> for Example33 {
	type Props = Example33Props;

	fn render(mut ctx: Ctx<'_>, props: Example33Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("isolate"), NoStyle::Noop("overflow-hidden"), Style::Color(Color::Fg(100)), Style::PaddingY(96), Screen::Small(&[Style::PaddingY(128)])]);
			
			props.child("1", Dynamic).run("img", |props| {
				props.set_attribute("src", "https://images.unsplash.com/photo-1521737604893-d14cc237f11d?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&crop=focalpoint&fp-y=.8&w=2830&h=1500&q=80&blend=111827&sat=-100&exp=15&blend-mode=multiply");
				props.set_attribute("alt", "");
				props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("-z-10"), NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-right"), Screen::Medium(&[NoStyle::Noop("object-center")])]);
			});
			props.child("3", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("hidden"), Screen::Small(&[NoStyle::Noop("absolute")]), Screen::Small(&[NoStyle::Noop("-top-10")]), Screen::Small(&[NoStyle::Noop("right-1/2")]), Screen::Small(&[NoStyle::Noop("-z-10")]), Screen::Small(&[Style::MarginRight(40)]), Screen::Small(&[Style::Block]), Screen::Small(&[NoStyle::Noop("transform-gpu")]), Screen::Small(&[NoStyle::Noop("blur-3xl")])]);
				props.set_attribute("aria-hidden", "true");
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("aspect-[1097/845]"), NoStyle::Noop("w-[68.5625rem]"), NoStyle::Noop("bg-gradient-to-tr"), NoStyle::Noop("from-[#ff4694]"), NoStyle::Noop("to-[#776fff]"), NoStyle::Noop("opacity-20")]);
					props.set_attribute("style", "clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)");
				});
			});
			props.child("5", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("-top-52"), NoStyle::Noop("left-1/2"), NoStyle::Noop("-z-10"), NoStyle::Noop("-translate-x-1/2"), NoStyle::Noop("transform-gpu"), NoStyle::Noop("blur-3xl"), Screen::Small(&[NoStyle::Noop("top-[-28rem]")]), Screen::Small(&[Style::MarginLeft(64)]), Screen::Small(&[NoStyle::Noop("translate-x-0")]), Screen::Small(&[NoStyle::Noop("transform-gpu")])]);
				props.set_attribute("aria-hidden", "true");
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("aspect-[1097/845]"), NoStyle::Noop("w-[68.5625rem]"), NoStyle::Noop("bg-gradient-to-tr"), NoStyle::Noop("from-[#ff4694]"), NoStyle::Noop("to-[#776fff]"), NoStyle::Noop("opacity-20")]);
					props.set_attribute("style", "clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)");
				});
			});
			props.child("7", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(24), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-2xl"), Screen::Large(&[Style::MarginX(0)])]);
					
					props.child("1", Dynamic).run("h2", |props| {
						props.styles(&[NoStyle::Noop("text-4xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), NoStyle::Noop("text-white"), Screen::Small(&[NoStyle::Noop("text-6xl")])]);
						
						props.child("0", Label).run(|props| props.set_text("Work with us"));
					});
					props.child("3", Dynamic).run("p", |props| {
						props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-lg"), NoStyle::Noop("leading-8"), Style::TextColor(Color::Fg(33))]);
						
						props.child("0", Label).run(|props| props.set_text("Anim aute id magna aliqua ad ad non deserunt sunt. Qui irure qui lorem cupidatat commodo. Elit sunt amet fugiat veniam occaecat fugiat aliqua."));
					});
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), Style::MarginTop(40), NoStyle::Noop("max-w-2xl"), Screen::Large(&[Style::MarginX(0)]), Screen::Large(&[NoStyle::Noop("max-w-none")])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-6"), NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), NoStyle::Noop("text-white"), Screen::Small(&[NoStyle::Noop("grid-cols-2")]), Screen::Medium(&[Style::Flex]), Screen::Large(&[NoStyle::Noop("gap-x-10")])]);
						
						props.child("1", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							
							props.child("0", Label).run(|props| props.set_text("Open roles"));
							props.child("1", Dynamic).run("span", |props| {
								props.set_attribute("aria-hidden", "true");
								
								props.child("0", Label).run(|props| props.set_text("→"));
							});
						});
						props.child("3", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							
							props.child("0", Label).run(|props| props.set_text("Internship program"));
							props.child("1", Dynamic).run("span", |props| {
								props.set_attribute("aria-hidden", "true");
								
								props.child("0", Label).run(|props| props.set_text("→"));
							});
						});
						props.child("5", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							
							props.child("0", Label).run(|props| props.set_text("Our values"));
							props.child("1", Dynamic).run("span", |props| {
								props.set_attribute("aria-hidden", "true");
								
								props.child("0", Label).run(|props| props.set_text("→"));
							});
						});
						props.child("7", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							
							props.child("0", Label).run(|props| props.set_text("Meet our leadership"));
							props.child("1", Dynamic).run("span", |props| {
								props.set_attribute("aria-hidden", "true");
								
								props.child("0", Label).run(|props| props.set_text("→"));
							});
						});
					});
					props.child("3", Dynamic).run("dl", |props| {
						props.styles(&[Style::MarginTop(64), NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-8"), Screen::Small(&[Style::MarginTop(80)]), Screen::Small(&[NoStyle::Noop("grid-cols-2")]), Screen::Large(&[NoStyle::Noop("grid-cols-4")])]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("flex-col-reverse")]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(33))]);
								
								props.child("0", Label).run(|props| props.set_text("Offices worldwide"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[NoStyle::Noop("text-2xl"), Style::FontBold, NoStyle::Noop("leading-9"), NoStyle::Noop("tracking-tight"), NoStyle::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.set_text("12"));
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("flex-col-reverse")]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(33))]);
								
								props.child("0", Label).run(|props| props.set_text("Full-time colleagues"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[NoStyle::Noop("text-2xl"), Style::FontBold, NoStyle::Noop("leading-9"), NoStyle::Noop("tracking-tight"), NoStyle::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.set_text("300+"));
							});
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("flex-col-reverse")]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(33))]);
								
								props.child("0", Label).run(|props| props.set_text("Hours per week"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[NoStyle::Noop("text-2xl"), Style::FontBold, NoStyle::Noop("leading-9"), NoStyle::Noop("tracking-tight"), NoStyle::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.set_text("40"));
							});
						});
						props.child("7", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("flex-col-reverse")]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(33))]);
								
								props.child("0", Label).run(|props| props.set_text("Paid time off"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[NoStyle::Noop("text-2xl"), Style::FontBold, NoStyle::Noop("leading-9"), NoStyle::Noop("tracking-tight"), NoStyle::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.set_text("Unlimited"));
							});
						});
					});
				});
			});
		});
	}
}
