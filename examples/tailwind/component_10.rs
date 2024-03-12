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
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("min-h-[500px]"), Style::Color(Color::Fg(11))]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.set_attribute("x-data", "{ open: true }");
				props.set_attribute("@keydown.window.escape", "open = false");
				props.set_attribute("x-init", r#"$watch("open", o => !o && window.setTimeout(() => (open = true), 1000))"#);
				props.set_attribute("x-show", "open");
				props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("z-10")]);
				props.set_attribute("aria-labelledby", "slide-over-title");
				props.set_attribute("x-ref", "dialog");
				props.set_attribute("aria-modal", "true");
				
				props.child("1", Dynamic).run("div", |props| {
					props.set_attribute("x-show", "open");
					props.set_attribute("x-transition:enter", "ease-in-out duration-500");
					props.set_attribute("x-transition:enter-start", "opacity-0");
					props.set_attribute("x-transition:enter-end", "opacity-100");
					props.set_attribute("x-transition:leave", "ease-in-out duration-500");
					props.set_attribute("x-transition:leave-start", "opacity-100");
					props.set_attribute("x-transition:leave-end", "opacity-0");
					props.set_attribute("x-description", "Background backdrop, show/hide based on slide-over state.");
					props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), Style::Color(Color::Fg(56)), NoStyle::Noop("bg-opacity-75"), NoStyle::Noop("transition-opacity")]);
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), NoStyle::Noop("overflow-hidden")]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("overflow-hidden")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("fixed"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, NoStyle::Noop("max-w-full"), Style::PaddingLeft(40)]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.set_attribute("x-show", "open");
								props.set_attribute("x-transition:enter", "transform transition ease-in-out duration-500 sm:duration-700");
								props.set_attribute("x-transition:enter-start", "translate-x-full");
								props.set_attribute("x-transition:enter-end", "translate-x-0");
								props.set_attribute("x-transition:leave", "transform transition ease-in-out duration-500 sm:duration-700");
								props.set_attribute("x-transition:leave-start", "translate-x-0");
								props.set_attribute("x-transition:leave-end", "translate-x-full");
								props.styles(&[NoStyle::Noop("pointer-events-auto"), NoStyle::Noop("relative"), NoStyle::Noop("w-screen"), NoStyle::Noop("max-w-md")]);
								props.set_attribute("x-description", "Slide-over panel, show/hide based on slide-over state.");
								props.set_attribute("@click.away", "open = false");
								
								props.child("1", Dynamic).run("div", |props| {
									props.set_attribute("x-show", "open");
									props.set_attribute("x-transition:enter", "ease-in-out duration-500");
									props.set_attribute("x-transition:enter-start", "opacity-0");
									props.set_attribute("x-transition:enter-end", "opacity-100");
									props.set_attribute("x-transition:leave", "ease-in-out duration-500");
									props.set_attribute("x-transition:leave-start", "opacity-100");
									props.set_attribute("x-transition:leave-end", "opacity-0");
									props.set_attribute("x-description", "Close button, show/hide based on slide-over state.");
									props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-0"), NoStyle::Noop("top-0"), NoStyle::Noop("-ml-8"), Style::Flex, Style::PaddingRight(8), Style::PaddingTop(16), Screen::Small(&[NoStyle::Noop("-ml-10")]), Screen::Small(&[Style::PaddingRight(16)])]);
									
									props.child("1", Dynamic).run("button", |props| {
										props.set_attribute("type", "button");
										props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("rounded-md"), Style::TextColor(Color::Fg(33)), Action::Hover(&[NoStyle::Noop("text-white")]), Action::Hover(&[NoStyle::Noop("outline-none")]), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-white")])]);
										props.set_attribute("@click", "open = false");
										
										props.child("1", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("-inset-2.5")]);
										});
										props.child("3", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("sr-only")]);
											
											props.child("0", Label).run(|props| props.set_text("Close panel"));
										});
										props.child("5", Icon).run(|props| {
											props.style(&[Style::Width(24), Style::Width(24)]);
										});
									});
								});
								props.child("3", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, NoStyle::Noop("h-full"), NoStyle::Noop("flex-col"), NoStyle::Noop("overflow-y-scroll"), NoStyle::Noop("bg-white"), Style::PaddingY(24), NoStyle::Noop("shadow-xl")]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::PaddingX(16), Screen::Small(&[Style::PaddingX(24)])]);
										
										props.child("1", Dynamic).run("h2", |props| {
											props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											props.set_attribute("id", "slide-over-title");
											
											props.child("0", Label).run(|props| props.set_text("Panel title"));
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("relative"), Style::MarginTop(24), NoStyle::Noop("flex-1"), Style::PaddingX(16), Screen::Small(&[Style::PaddingX(24)])]);
										
										props.child("1", Dynamic).run("x-placeholder", |props| {
											props.child("1", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), Style::PaddingX(16), Screen::Small(&[Style::PaddingX(24)])]);
												
												props.child("1", Dynamic).run("div", |props| {
													props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("h-full"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-xl"), NoStyle::Noop("border"), NoStyle::Noop("border-dashed"), NoStyle::Noop("border-gray-400"), NoStyle::Noop("opacity-75")]);
													
													props.child("1", Icon).run(|props| {
														props.style(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("stroke-gray-900/10")]);
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
