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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("relative"), Style::Noop("isolate"), Style::Noop("overflow-hidden"), Style::Color(Color::Fg(100)), Style::PaddingY(Size::Exact(64)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(96))]), Style::OnScreen(Screen::Large, &[Style::PaddingY(Size::Exact(128))])]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(24)), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Noop("grid"), Style::Noop("max-w-2xl"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-8"), Style::Noop("gap-y-16"), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-none")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-2")])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("max-w-xl"), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-lg")])]);
						
						props.child("1", Dynamic).run("h2", |mut props| {
							props.styles(&[Style::Noop("text-3xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::Noop("text-white"), Style::OnScreen(Screen::Small, &[Style::Noop("text-4xl")])]);
							
							props.child("0", Label).run(|props| props.text("Subscribe to our newsletter."));
						});
						props.child("3", Dynamic).run("p", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(16)), Style::Noop("text-lg"), Style::Noop("leading-8"), Style::TextColor(Color::Fg(33))]);
							
							props.child("0", Label).run(|props| props.text("Nostrud amet eu ullamco nisi aute in ad minim nostrud adipisicing velit quis. Duis tempor incididunt dolore."));
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Flex, Style::Noop("max-w-md"), Style::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("label", |mut props| {
								props.set_attribute("for", "email-address");
								props.styles(&[Style::Noop("sr-only")]);
								
								props.child("0", Label).run(|props| props.text("Email address"));
							});
							props.child("3", Dynamic).run("input", |mut props| {
								props.set_attribute("id", "email-address");
								props.set_attribute("name", "email");
								props.set_attribute("type", "email");
								props.set_attribute("autocomplete", "email");
								props.set_attribute("required", "");
								props.styles(&[Style::Noop("min-w-0"), Style::Noop("flex-auto"), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::Noop("bg-white/5"), Style::PaddingX(Size::Exact(14)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-white"), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-white/10"), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-500")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
								props.set_attribute("placeholder", "Enter your email");
							});
							props.child("5", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "submit");
								props.styles(&[Style::Noop("flex-none"), Style::Noop("rounded-md"), Style::Noop("bg-indigo-500"), Style::PaddingX(Size::Exact(14)), Style::PaddingY(Size::Exact(10)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("text-white"), Style::Noop("shadow-sm"), Style::OnHover(&[Style::Noop("bg-indigo-400")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-offset-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-indigo-500")])]);
								
								props.child("0", Label).run(|props| props.text("Subscribe"));
							});
						});
					});
					props.child("3", Dynamic).run("dl", |mut props| {
						props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-8"), Style::Noop("gap-y-10"), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-2")]), Style::OnScreen(Screen::Large, &[Style::PaddingTop(Size::Exact(8))])]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("flex-col"), Style::Noop("items-start")]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("rounded-md"), Style::Noop("bg-white/5"), Style::Padding(Size::Exact(8)), Style::Noop("ring-1"), Style::Noop("ring-white/10")]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::Noop("text-white")]);
								});
							});
							props.child("3", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(16)), Style::FontSemibold, Style::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.text("Weekly articles"));
							});
							props.child("5", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("leading-7"), Style::TextColor(Color::Fg(44))]);
								
								props.child("0", Label).run(|props| props.text("Non laboris consequat cupidatat laborum magna. Eiusmod non irure cupidatat duis commodo amet."));
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Noop("flex-col"), Style::Noop("items-start")]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("rounded-md"), Style::Noop("bg-white/5"), Style::Padding(Size::Exact(8)), Style::Noop("ring-1"), Style::Noop("ring-white/10")]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::Noop("text-white")]);
								});
							});
							props.child("3", Dynamic).run("dt", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(16)), Style::FontSemibold, Style::Noop("text-white")]);
								
								props.child("0", Label).run(|props| props.text("No spam"));
							});
							props.child("5", Dynamic).run("dd", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("leading-7"), Style::TextColor(Color::Fg(44))]);
								
								props.child("0", Label).run(|props| props.text("Officia excepteur ullamco ut sint duis proident non adipisicing. Voluptate incididunt anim."));
							});
						});
					});
				});
			});
			props.child("3", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("absolute"), Style::Noop("left-1/2"), Style::Noop("top-0"), Style::Noop("-z-10"), Style::Noop("-translate-x-1/2"), Style::Noop("blur-3xl"), Style::OnScreen(Screen::ExtraLarge(1), &[Style::Noop("-top-6")])]);
				props.set_attribute("aria-hidden", "true");
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("aspect-[1155/678]"), Style::Noop("w-[72.1875rem]"), Style::Noop("bg-gradient-to-tr"), Style::Noop("from-[#ff80b5]"), Style::Noop("to-[#9089fc]"), Style::Noop("opacity-30")]);
					props.set_attribute("style", "clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)");
				});
			});
		});
	}
}
