use rust_web_ui::*;

pub struct Example13;

pub struct Example13Props {}

impl Default for Example13Props {
	fn default() -> Example13Props {
		Example13Props { }
	}
}

impl Widget<'_> for Example13 {
	type Props = Example13Props;

	fn render(mut ctx: Ctx<'_>, props: Example13Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Color(Color::Fg(33))]);
			props.set_attribute("style", "height: 560px");
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.set_attribute("x-data", "{ open: true }");
				props.set_attribute("@keydown.window.escape", "open = false");
				props.set_attribute("x-init", r#"$watch("open", o => !o && window.setTimeout(() => (open = true), 1000))"#);
				props.set_attribute("x-show", "open");
				props.styles(&[Style::Noop("relative"), Style::Noop("z-10")]);
				props.set_attribute("aria-labelledby", "modal-title");
				props.set_attribute("x-ref", "dialog");
				props.set_attribute("aria-modal", "true");
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.set_attribute("x-show", "open");
					props.set_attribute("x-transition:enter", "ease-out duration-300");
					props.set_attribute("x-transition:enter-start", "opacity-0");
					props.set_attribute("x-transition:enter-end", "opacity-100");
					props.set_attribute("x-transition:leave", "ease-in duration-200");
					props.set_attribute("x-transition:leave-start", "opacity-100");
					props.set_attribute("x-transition:leave-end", "opacity-0");
					props.set_attribute("x-description", "Background backdrop, show/hide based on modal state.");
					props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Color(Color::Fg(56)), Style::Noop("bg-opacity-75"), Style::Noop("transition-opacity")]);
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Noop("z-10"), Style::Noop("w-screen"), Style::Noop("overflow-y-auto")]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Flex, Style::Noop("min-h-full"), Style::Noop("items-end"), Style::JustifyCenter, Style::Padding(Size::Exact(16)), Style::Noop("text-center"), Style::OnScreen(Screen::Small, &[Style::ItemsCenter]), Style::OnScreen(Screen::Small, &[Style::Padding(Size::Exact(0))])]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.set_attribute("x-show", "open");
							props.set_attribute("x-transition:enter", "ease-out duration-300");
							props.set_attribute("x-transition:enter-start", "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95");
							props.set_attribute("x-transition:enter-end", "opacity-100 translate-y-0 sm:scale-100");
							props.set_attribute("x-transition:leave", "ease-in duration-200");
							props.set_attribute("x-transition:leave-start", "opacity-100 translate-y-0 sm:scale-100");
							props.set_attribute("x-transition:leave-end", "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95");
							props.set_attribute("x-description", "Modal panel, show/hide based on modal state.");
							props.styles(&[Style::Noop("relative"), Style::Noop("transform"), Style::Noop("overflow-hidden"), Style::Noop("rounded-lg"), Style::Noop("bg-white"), Style::Noop("text-left"), Style::Noop("shadow-xl"), Style::Noop("transition-all"), Style::OnScreen(Screen::Small, &[Style::MarginY(Size::Exact(32))]), Style::OnScreen(Screen::Small, &[Style::Width(Size::Full)]), Style::OnScreen(Screen::Small, &[Style::Noop("max-w-lg")])]);
							props.set_attribute("@click.away", "open = false");
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("bg-white"), Style::PaddingX(Size::Exact(16)), Style::PaddingBottom(Size::Exact(16)), Style::PaddingTop(Size::Exact(20)), Style::OnScreen(Screen::Small, &[Style::Padding(Size::Exact(24))]), Style::OnScreen(Screen::Small, &[Style::PaddingBottom(Size::Exact(16))])]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::OnScreen(Screen::Small, &[Style::Flex]), Style::OnScreen(Screen::Small, &[Style::Noop("items-start")])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("mx-auto"), Style::Flex, Style::Width(Size::Exact(48)), Style::Width(Size::Exact(48)), Style::Noop("flex-shrink-0"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-full"), Style::Color(Color::Danger(11)), Style::OnScreen(Screen::Small, &[Style::MarginX(Size::Exact(0))]), Style::OnScreen(Screen::Small, &[Style::Width(Size::Exact(40))]), Style::OnScreen(Screen::Small, &[Style::Width(Size::Exact(40))])]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::TextColor(Color::Danger(67))]);
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::MarginTop(Size::Exact(12)), Style::Noop("text-center"), Style::OnScreen(Screen::Small, &[Style::MarginLeft(Size::Exact(16))]), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(0))]), Style::OnScreen(Screen::Small, &[Style::Noop("text-left")])]);
										
										props.child("1", Dynamic).run("h3", |mut props| {
											props.styles(&[Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											props.set_attribute("id", "modal-title");
											
											props.child("0", Label).run(|props| props.text("Deactivate account"));
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(8))]);
											
											props.child("1", Dynamic).run("p", |mut props| {
												props.styles(&[Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
												
												props.child("0", Label).run(|props| props.text("Are you sure you want to deactivate your account? All of your data will be permanently removed. This action cannot be undone."));
											});
										});
									});
								});
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Color(Color::Fg(6)), Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(12)), Style::OnScreen(Screen::Small, &[Style::Flex]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-row-reverse")]), Style::OnScreen(Screen::Small, &[Style::PaddingX(Size::Exact(24))])]);
								
								props.child("1", Dynamic).run("button", |mut props| {
									props.set_attribute("type", "button");
									props.styles(&[Style::InlineFlex, Style::Width(Size::Full), Style::JustifyCenter, Style::Noop("rounded-md"), Style::Color(Color::Danger(67)), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("text-white"), Style::Noop("shadow-sm"), Style::OnHover(&[Style::Color(Color::Danger(56))]), Style::OnScreen(Screen::Small, &[Style::MarginLeft(Size::Exact(12))]), Style::OnScreen(Screen::Small, &[Style::Noop("w-auto")])]);
									props.set_attribute("@click", "open = false");
									
									props.child("0", Label).run(|props| props.text("Deactivate"));
								});
								props.child("3", Dynamic).run("button", |mut props| {
									props.set_attribute("type", "button");
									props.styles(&[Style::MarginTop(Size::Exact(12)), Style::InlineFlex, Style::Width(Size::Full), Style::JustifyCenter, Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Color(Color::Fg(6))]), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(0))]), Style::OnScreen(Screen::Small, &[Style::Noop("w-auto")])]);
									props.set_attribute("@click", "open = false");
									
									props.child("0", Label).run(|props| props.text("Cancel"));
								});
							});
						});
					});
				});
			});
		});
	}
}
