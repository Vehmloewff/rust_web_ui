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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("relative"), Style::Noop("isolate"), Style::Noop("overflow-hidden"), Style::Color(Color::Fg(100)), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))])]);
			
			props.child("1", Dynamic).run("img", |mut props| {
				props.set_attribute("src", "https://images.unsplash.com/photo-1521737604893-d14cc237f11d?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&crop=focalpoint&fp-y=.8&w=2830&h=1500&q=80&blend=111827&sat=-100&exp=15&blend-mode=multiply");
				props.set_attribute("alt", "");
				props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("-z-10"), Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("object-cover"), Style::Noop("object-right"), Style::OnScreen(Screen::Medium, &[Style::Noop("object-center")])]);
			});
			props.child("3", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("hidden"), Style::OnScreen(Screen::Small, &[Style::Noop("absolute")]), Style::OnScreen(Screen::Small, &[Style::Noop("-top-10")]), Style::OnScreen(Screen::Small, &[Style::Noop("right-1/2")]), Style::OnScreen(Screen::Small, &[Style::Noop("-z-10")]), Style::OnScreen(Screen::Small, &[Style::MarginRight(Size::Exact(40))]), Style::OnScreen(Screen::Small, &[Style::Block]), Style::OnScreen(Screen::Small, &[Style::Noop("transform-gpu")]), Style::OnScreen(Screen::Small, &[Style::Noop("blur-3xl")])]);
				props.set_attribute("aria-hidden", "true");
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("aspect-[1097/845]"), Style::Noop("w-[68.5625rem]"), Style::Noop("bg-gradient-to-tr"), Style::Noop("from-[#ff4694]"), Style::Noop("to-[#776fff]"), Style::Noop("opacity-20")]);
					props.set_attribute("style", "clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)");
				});
			});
			props.child("5", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("absolute"), Style::Noop("-top-52"), Style::Noop("left-1/2"), Style::Noop("-z-10"), Style::Noop("-translate-x-1/2"), Style::Noop("transform-gpu"), Style::Noop("blur-3xl"), Style::OnScreen(Screen::Small, &[Style::Noop("top-[-28rem]")]), Style::OnScreen(Screen::Small, &[Style::MarginLeft(Size::Exact(64))]), Style::OnScreen(Screen::Small, &[Style::Noop("translate-x-0")]), Style::OnScreen(Screen::Small, &[Style::Noop("transform-gpu")])]);
				props.set_attribute("aria-hidden", "true");
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("aspect-[1097/845]"), Style::Noop("w-[68.5625rem]"), Style::Noop("bg-gradient-to-tr"), Style::Noop("from-[#ff4694]"), Style::Noop("to-[#776fff]"), Style::Noop("opacity-20")]);
					props.set_attribute("style", "clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)");
				});
			});
			props.child("7", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(24)), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-2xl"), Style::OnScreen(Screen::Large, &[Style::MarginX(Size::Exact(0))])]);
					
					props.child("1", Dynamic).run("h2", |mut props| {
						props.styles(&[Style::Noop("text-4xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::Noop("text-white"), Style::OnScreen(Screen::Small, &[Style::Noop("text-6xl")])]);
						
						props.child("0", Label).run(|props| props.text("Work with us"));
					});
					props.child("3", Dynamic).run("p", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("text-lg"), Style::Noop("leading-8"), Style::TextColor(Color::Fg(33))]);
						
						props.child("0", Label).run(|props| props.text("Anim aute id magna aliqua ad ad non deserunt sunt. Qui irure qui lorem cupidatat commodo. Elit sunt amet fugiat veniam occaecat fugiat aliqua."));
					});
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::MarginTop(Size::Exact(40)), Style::Noop("max-w-2xl"), Style::OnScreen(Screen::Large, &[Style::MarginX(Size::Exact(0))]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-none")])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-8"), Style::Noop("gap-y-6"), Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::Noop("text-white"), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Medium, &[Style::Flex]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-x-10")])]);
						
						props.child("1", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							
							props.child("0", Label).run(|props| props.text("Open roles"));
							props.child("1", Dynamic).run("span", |mut props| {
								props.set_attribute("aria-hidden", "true");
								
								props.child("0", Label).run(|props| props.text("→"));
							});
						});
						props.child("3", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							
							props.child("0", Label).run(|props| props.text("Internship program"));
							props.child("1", Dynamic).run("span", |mut props| {
								props.set_attribute("aria-hidden", "true");
								
								props.child("0", Label).run(|props| props.text("→"));
							});
						});
						props.child("5", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							
							props.child("0", Label).run(|props| props.text("Our values"));
							props.child("1", Dynamic).run("span", |mut props| {
								props.set_attribute("aria-hidden", "true");
								
								props.child("0", Label).run(|props| props.text("→"));
							});
						});
						props.child("7", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							
							props.child("0", Label).run(|props| props.text("Meet our leadership"));
							props.child("1", Dynamic).run("span", |mut props| {
								props.set_attribute("aria-hidden", "true");
								
								props.child("0", Label).run(|props| props.text("→"));
							});
						});
					});
					props.child("3", Dynamic).run("dl", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(64)), Style::Noop("grid"), Style::Noop("grid-cols-1"), Style::Noop("gap-8"), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(80))]), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-4")])]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("flex-col-reverse")]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(33))]);
								
								props.child("0", Label).run(|props| props.text("Offices worldwide"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::Noop("text-2xl"), Style::FontBold, Style::Noop("leading-9"), Style::Noop("tracking-tight"), Style::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.text("12"));
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("flex-col-reverse")]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(33))]);
								
								props.child("0", Label).run(|props| props.text("Full-time colleagues"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::Noop("text-2xl"), Style::FontBold, Style::Noop("leading-9"), Style::Noop("tracking-tight"), Style::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.text("300+"));
							});
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("flex-col-reverse")]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(33))]);
								
								props.child("0", Label).run(|props| props.text("Hours per week"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::Noop("text-2xl"), Style::FontBold, Style::Noop("leading-9"), Style::Noop("tracking-tight"), Style::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.text("40"));
							});
						});
						props.child("7", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("flex-col-reverse")]);
							
							props.child("1", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(33))]);
								
								props.child("0", Label).run(|props| props.text("Paid time off"));
							});
							props.child("3", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::Noop("text-2xl"), Style::FontBold, Style::Noop("leading-9"), Style::Noop("tracking-tight"), Style::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.text("Unlimited"));
							});
						});
					});
				});
			});
		});
	}
}
