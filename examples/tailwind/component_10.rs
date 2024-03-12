use rust_web_ui::*;

pub struct Example10;

pub struct Example10Props {}

impl Default for Example10Props {
	fn default() -> Example10Props {
		Example10Props { }
	}
}

impl Widget<'_> for Example10 {
	type Props = Example10Props;

	fn render(mut ctx: Ctx<'_>, props: Example10Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("min-h-[500px]"), Style::Color(Color::Fg(11))]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.set_attribute("x-data", "{ open: true }");
				props.set_attribute("@keydown.window.escape", "open = false");
				props.set_attribute("x-init", r#"$watch("open", o => !o && window.setTimeout(() => (open = true), 1000))"#);
				props.set_attribute("x-show", "open");
				props.styles(&[Style::Noop("relative"), Style::Noop("z-10")]);
				props.set_attribute("aria-labelledby", "slide-over-title");
				props.set_attribute("x-ref", "dialog");
				props.set_attribute("aria-modal", "true");
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.set_attribute("x-show", "open");
					props.set_attribute("x-transition:enter", "ease-in-out duration-500");
					props.set_attribute("x-transition:enter-start", "opacity-0");
					props.set_attribute("x-transition:enter-end", "opacity-100");
					props.set_attribute("x-transition:leave", "ease-in-out duration-500");
					props.set_attribute("x-transition:leave-start", "opacity-100");
					props.set_attribute("x-transition:leave-end", "opacity-0");
					props.set_attribute("x-description", "Background backdrop, show/hide based on slide-over state.");
					props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Color(Color::Fg(56)), Style::Noop("bg-opacity-75"), Style::Noop("transition-opacity")]);
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Noop("overflow-hidden")]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Noop("overflow-hidden")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("fixed"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::Noop("max-w-full"), Style::PaddingLeft(Size::Exact(40))]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.set_attribute("x-show", "open");
								props.set_attribute("x-transition:enter", "transform transition ease-in-out duration-500 sm:duration-700");
								props.set_attribute("x-transition:enter-start", "translate-x-full");
								props.set_attribute("x-transition:enter-end", "translate-x-0");
								props.set_attribute("x-transition:leave", "transform transition ease-in-out duration-500 sm:duration-700");
								props.set_attribute("x-transition:leave-start", "translate-x-0");
								props.set_attribute("x-transition:leave-end", "translate-x-full");
								props.styles(&[Style::Noop("pointer-events-auto"), Style::Noop("relative"), Style::Noop("w-screen"), Style::Noop("max-w-md")]);
								props.set_attribute("x-description", "Slide-over panel, show/hide based on slide-over state.");
								props.set_attribute("@click.away", "open = false");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.set_attribute("x-show", "open");
									props.set_attribute("x-transition:enter", "ease-in-out duration-500");
									props.set_attribute("x-transition:enter-start", "opacity-0");
									props.set_attribute("x-transition:enter-end", "opacity-100");
									props.set_attribute("x-transition:leave", "ease-in-out duration-500");
									props.set_attribute("x-transition:leave-start", "opacity-100");
									props.set_attribute("x-transition:leave-end", "opacity-0");
									props.set_attribute("x-description", "Close button, show/hide based on slide-over state.");
									props.styles(&[Style::Noop("absolute"), Style::Noop("left-0"), Style::Noop("top-0"), Style::Noop("-ml-8"), Style::Flex, Style::PaddingRight(Size::Exact(8)), Style::PaddingTop(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::Noop("-ml-10")]), Style::OnScreen(Screen::Small, &[Style::PaddingRight(Size::Exact(16))])]);
									
									props.child("1", Dynamic).run("button", |mut props| {
										props.set_attribute("type", "button");
										props.styles(&[Style::Noop("relative"), Style::Noop("rounded-md"), Style::TextColor(Color::Fg(33)), Style::OnHover(&[Style::Noop("text-white")]), Style::OnFocus(&[Style::Noop("outline-none")]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-white")])]);
										props.set_attribute("@click", "open = false");
										
										props.child("1", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("absolute"), Style::Noop("-inset-2.5")]);
										});
										props.child("3", Dynamic).run("span", |mut props| {
											props.styles(&[Style::Noop("sr-only")]);
											
											props.child("0", Label).run(|props| props.text("Close panel"));
										});
										props.child("5", Icon).run(|mut props| {
											props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24))]);
										});
									});
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Flex, Style::Width(Size::Full), Style::Noop("flex-col"), Style::Noop("overflow-y-scroll"), Style::Noop("bg-white"), Style::PaddingY(Size::Exact(24)), Style::Noop("shadow-xl")]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::PaddingX(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))])]);
										
										props.child("1", Dynamic).run("h2", |mut props| {
											props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											props.set_attribute("id", "slide-over-title");
											
											props.child("0", Label).run(|props| props.text("Panel title"));
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("relative"), Style::MarginTop(Size::Exact(24)), Style::Noop("flex-1"), Style::PaddingX(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))])]);
										
										props.child("1", Dynamic).run("x-placeholder", |mut props| {
											props.child("1", Dynamic).run("div", |mut props| {
												props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::PaddingX(Size::Exact(16)), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))])]);
												
												props.child("1", Dynamic).run("div", |mut props| {
													props.styles(&[Style::Noop("relative"), Style::Width(Size::Full), Style::Noop("overflow-hidden"), Style::Noop("rounded-xl"), Style::Noop("border"), Style::Noop("border-dashed"), Style::Noop("border-gray-400"), Style::Noop("opacity-75")]);
													
													props.child("1", Icon).run(|mut props| {
														props.style(&[Style::Noop("absolute"), Style::Noop("inset-0"), Style::Width(Size::Full), Style::Width(Size::Full), Style::Noop("stroke-gray-900/10")]);
													});
												});
											});
										});
									});
								});
							});
						});
					});
				});
			});
		});
	}
}
